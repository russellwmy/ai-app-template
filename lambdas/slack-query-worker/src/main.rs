use aws_lambda_events::dynamodb::Event;
use composer::{
    utils::{compose_message_with_graph, get_composer},
    Composer,
};
use database::{
    delete_task,
    from_item,
    get_collection,
    get_collection_id_from_slack_channel_id,
    get_slack_team,
    put_task,
    SlackMessageTask,
    Task,
    TaskKind,
};
use indexer::{
    utils::{get_embedding_model, load_graphs_from_s3, search_graph},
    EmbeddingModel,
};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use tracing::info;

struct Context {
    s3_client: aws_sdk_s3::Client,
    dynamodb_client: aws_sdk_dynamodb::Client,
    model: EmbeddingModel,
    composer: Composer,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    //Get config from environment.
    let resources_path = common::vars::get_app_resources_path()?;
    let config = aws_config::load_from_env().await;
    let s3_client = aws_sdk_s3::Client::new(&config);
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);
    let model = get_embedding_model(&resources_path).await?;
    let composer = get_composer(2000).await?;
    let context = Context {
        s3_client,
        dynamodb_client,
        model,
        composer,
    };
    let context_ref = &context;
    run(service_fn(move |event: LambdaEvent<Event>| async move {
        Ok::<(), Error>(process_request(event, context_ref).await?)
    }))
    .await?;
    Ok(())
}

async fn process_request(event: LambdaEvent<Event>, context: &Context) -> Result<(), Error> {
    if let Some(record) = event.payload.records.last() {
        info!("Record: {:?}", record);
        if record.event_name == "INSERT" {
            let task: Task = from_item(record.change.new_image.clone())?;
            let task_id = task.id();
            match task.kind() {
                TaskKind::SlackQueryTask(task) => {
                    let slack_team =
                        get_slack_team(&context.dynamodb_client, task.slack_team_id()).await?;
                    let collection_id = get_collection_id_from_slack_channel_id(
                        &context.dynamodb_client,
                        &task.slack_user_id(),
                        &slack_team,
                        &task.slack_channel_id(),
                    )
                    .await?;
                    let collection =
                        get_collection(&context.dynamodb_client, &collection_id).await?;
                    let bucket_name = common::vars::get_app_document_bucket()?;
                    let document_keys = collection.documents().iter().map(|k| k.as_str()).collect();
                    let graphs =
                        load_graphs_from_s3(&context.s3_client, &bucket_name, document_keys)
                            .await?;
                    let nodes = search_graph(graphs, &task.text(), &context.model).await?;
                    let (message, _) =
                        compose_message_with_graph(&context.composer, nodes, &task.text(), 1500)
                            .await?;
                    let message_task = SlackMessageTask::builder()
                        .slack_team_id(task.slack_team_id().to_string())
                        .slack_channel_id(task.slack_channel_id().to_string())
                        .slack_user_id(task.slack_user_id().to_string())
                        .text(message)
                        .build();
                    let task = Task::builder().kind(message_task.into()).build();
                    put_task(&context.dynamodb_client, task).await?;
                    delete_task(&context.dynamodb_client, task_id).await?;
                }

                _ => {}
            };
        }
    }

    Ok(())
}
