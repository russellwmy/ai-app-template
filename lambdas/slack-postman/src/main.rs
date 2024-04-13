use aws_lambda_events::dynamodb::Event;
use database::{delete_task, from_item, get_slack_team, Task, TaskKind};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use slack::SlackOAuthV2Info;
use slack_morphism::prelude::*;
use tracing::{debug, info};

struct Context {
    slack_client: SlackHyperClient,
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

    let slack_client = SlackClient::new(SlackClientHyperConnector::new());
    //Get config from environment.
    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);
    let context = Context {
        slack_client,
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
            let task_id = task.id();
            match task.kind() {
                TaskKind::SlackMessageTask(message_task) => {
                    let slack_team_id = message_task.slack_team_id();
                    let slack_user_id = message_task.slack_user_id();
                    let slack_channel_id = message_task.slack_channel_id();
                    let text = message_task.text();
                    let slack_team =
                        get_slack_team(&context.dynamodb_client, slack_team_id).await?;
                    let slack_oauth_info: SlackOAuthV2Info =
                        serde_json::from_str(slack_team.oauth_data())?;
                    let token = SlackApiToken::new(slack_oauth_info.access_token);
                    let content = format!("<@{}> {}", slack_user_id, text);
                    slack::chat::post(
                        &context.slack_client,
                        token,
                        slack_channel_id.into(),
                        &content,
                    )
                    .await
                    .map_err(|e| format!("fail to send slack message. {}", e.to_string()))?;

                    info!(
                        event = "message_delivered",
                        content = format!("{:?}", content),
                        organization_id = slack_team.organization_id()
                    );

                    debug!("processed task: {}", task_id);
                    delete_task(&context.dynamodb_client, task_id).await?;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
