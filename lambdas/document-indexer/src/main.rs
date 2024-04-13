use aws_config::BehaviorVersion;
use aws_lambda_events::dynamodb::Event;
use database::{
    delete_task, from_item, get_document, put_task, update_document_index_state, CallbackTask,
    Task, TaskKind,
};
use indexer::IndexingMeta;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde_json::json;

struct Context {
    s3_client: aws_sdk_s3::Client,
    dynamodb_client: aws_sdk_dynamodb::Client,
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
    let config = aws_config::load_defaults(BehaviorVersion::v2023_11_09()).await;
    let s3_client = aws_sdk_s3::Client::new(&config);
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);
    let context = Context {
        s3_client,
        dynamodb_client,
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
        if record.event_name == "INSERT" {
            let task: Task = from_item(record.change.new_image.clone())?;
            let task_id = task.id().to_string();
            match task.kind() {
                TaskKind::IndexingTask(task) => {
                    let document_id = task.document_id().to_owned();
                    let file_key = task.file_key().to_owned();
                    let external_link = task.external_link().clone().unwrap_or("".to_string());
                    let meta = IndexingMeta::builder()
                        .id(common::generate_id())
                        .title(task.filename().to_string())
                        .external_link(external_link)
                        .build();

                    indexer::utils::build_index(
                        &context.s3_client,
                        &meta,
                        &document_id,
                        &file_key,
                        true,
                    )
                    .await
                    .map_err(|e| anyhow::anyhow!(e.to_string()))?;
                    update_document_index_state(&context.dynamodb_client, &document_id, "ready")
                        .await?;
                    delete_task(&context.dynamodb_client, &task_id).await?;
                    if let Some(callback_url) = task.callback_url() {
                        let document = get_document(&context.dynamodb_client, &document_id).await?;
                        let data = json!({
                            "document_id": document_id,
                            "index_state": document.index_state(),
                            "status": document.status()
                        });
                        let callback_task = CallbackTask::builder()
                            .data(data)
                            .callback_url(callback_url.to_string())
                            .caller_id(task.creator_id().to_string())
                            .build();
                        let task = Task::builder().kind(callback_task.into()).build();
                        put_task(&context.dynamodb_client, task).await?;
                    }
                }

                _ => {}
            };
        }
    }

    Ok(())
}
