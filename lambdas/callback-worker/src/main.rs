use std::collections::HashMap;

use aws_config::BehaviorVersion;
use aws_lambda_events::dynamodb::Event;
use database::{delete_task, from_item, CallbackTask, Task, TaskKind};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde_json::json;
use tracing::{error, info};
struct Context {
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
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);
    let context = Context { dynamodb_client };
    let context_ref = &context;
    run(service_fn(move |event: LambdaEvent<Event>| async move {
        Ok::<(), Error>(process_request(event, context_ref).await?)
    }))
    .await?;
    Ok(())
}

async fn process_request(event: LambdaEvent<Event>, context: &Context) -> Result<(), Error> {
    if let Some(record) = event.payload.records.last() {
        let task: Task = from_item(record.change.new_image.clone())?;
        let task_id = task.id().to_string();

        match task.kind() {
            TaskKind::CallbackTask(task) => {
                let result = process_task(task, context).await;
                match result {
                    Ok(_) => delete_task(&context.dynamodb_client, &task_id).await?,
                    Err(e) => {
                        error!("{}", e.to_string());
                        delete_task(&context.dynamodb_client, &task_id).await?;
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}

async fn process_task(task: &CallbackTask, _: &Context) -> Result<(), Error> {
    let callback_url_full = common::clean_url(task.callback_url());
    if let Ok(url_object) = common::parse_url(&callback_url_full) {
        let callback_url_parts = callback_url_full.split("?").collect::<Vec<&str>>();
        let callback_url = *callback_url_parts.first().unwrap();

        let mut state = HashMap::new();
        for (k, v) in url_object.query_pairs() {
            state.insert(k, v);
        }

        let data = json!({
            "caller_id":  task.caller_id().clone(),
            "state":   serde_json::to_value(state)?,
            "result":  task.data().clone(),
        });

        let client = reqwest::Client::new();
        info!("send request to: {}", callback_url);
        info!("data: {:?}", data);
        client.post(callback_url).json(&data).send().await?;
    }

    Ok(())
}
