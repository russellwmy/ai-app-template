use database::{
    get_slack_team,
    get_user_id_from_slack_user_id,
    put_task,
    SlackDownloadTask,
    SlackQueryTask,
    Task,
};
use lambda_http::{Body, Request, RequestExt};
use route_recognizer::Router;
use slack_morphism::prelude::{SlackCommandEvent, SlackEventCallbackBody, SlackPushEvent};
use tracing::info;

use crate::Context;

enum PostRoutes {
    SlackEvents,
    SlackCommands,
}

pub(crate) async fn process_post_request(
    request: Request,
    context: &Context,
) -> anyhow::Result<Body> {
    let mut router = Router::new();
    router.add("/:stage/events", PostRoutes::SlackEvents);
    router.add("/:stage/commands", PostRoutes::SlackCommands);
    let routing = router.recognize(&request.raw_http_path());

    match routing {
        Ok(routing) => match routing.handler() {
            PostRoutes::SlackEvents => {
                slack::verify_api_gateway_req(&request);
                let event = request
                    .payload::<SlackPushEvent>()
                    .expect("unable to deserialize")
                    .expect("no body provided");
                process_events(event, context).await
            }
            PostRoutes::SlackCommands => {
                slack::verify_api_gateway_req(&request);
                let event = request
                    .payload::<SlackCommandEvent>()
                    .expect("unable to deserialize")
                    .expect("no body provided");

                let learn_command_name = get_slack_learn_command().unwrap();
                if event.command.to_string() == learn_command_name {
                    process_learn_command(event, &context).await
                } else {
                    Err(anyhow::anyhow!(format!(
                        "Unknown command: {}",
                        learn_command_name
                    )))
                }
            }
            _ => Err(anyhow::anyhow!("Not found")),
        },
        Err(_) => Err(anyhow::anyhow!("Not found")),
    }
}

async fn process_events(event: SlackPushEvent, context: &Context) -> anyhow::Result<Body> {
    match event {
        SlackPushEvent::EventCallback(req) => {
            let slack_team =
                get_slack_team(&context.dynamodb_client, &req.team_id.to_string()).await?;

            match req.event {
                SlackEventCallbackBody::AppMention(event) => {
                    let slack_user_id = event.user.to_string();
                    let user_id = get_user_id_from_slack_user_id(
                        &context.dynamodb_client,
                        &slack_team,
                        &slack_user_id,
                    )
                    .await?;
                    info!(
                        event = "app_mentioned",
                        organization_id = slack_team.organization_id(),
                        user_id = user_id
                    );
                    let content = event.content.text.unwrap_or("".to_string());
                    let contents = content.as_str().split_whitespace().collect::<Vec<&str>>();
                    if let Some((_, words)) = contents.split_first() {
                        let content = words.join(" ");
                        let query_task = SlackQueryTask::builder()
                            .slack_team_id(req.team_id.to_string())
                            .slack_channel_id(event.channel.to_string())
                            .slack_user_id(event.user.to_string())
                            .text(content)
                            .build();
                        let task = Task::builder().kind(query_task.into()).build();
                        put_task(&context.dynamodb_client, task).await?;
                    }

                    Ok("".into())
                }
                SlackEventCallbackBody::AppUninstalled(_) => {
                    info!(
                        event = "app_uninstalled",
                        organization_id = slack_team.organization_id()
                    );
                    Ok("".into())
                }
                _ => Ok("".into()),
            }
        }
        SlackPushEvent::UrlVerification(req) => {
            let challenge = req.challenge.as_str().to_string();
            info!("challenge: {}", challenge);

            Ok(challenge.into())
        }
        _ => Ok("".into()),
    }
}

fn get_slack_learn_command() -> Result<String, std::env::VarError> {
    let value = match common::vars::get_app_environment()? {
        common::Environment::Production => "/learn".to_string(),
        _ => "/learn-stage".to_string(),
    };
    Ok(value)
}

async fn process_learn_command(
    event: SlackCommandEvent,
    context: &Context,
) -> anyhow::Result<Body> {
    let slack_team = get_slack_team(&context.dynamodb_client, &event.team_id.to_string()).await?;

    let user_id = get_user_id_from_slack_user_id(
        &context.dynamodb_client,
        &slack_team,
        &event.user_id.to_string(),
    )
    .await?;
    info!(
        event = "new_file_uploaded",
        organization_id = slack_team.team_id(),
        user_id = user_id
    );

    match event.text {
        Some(text) => {
            if !text.is_empty() {
                let url = &text;
                let download_task = SlackDownloadTask::builder()
                    .slack_team_id(event.team_id.to_string())
                    .slack_channel_id(event.channel_id.to_string())
                    .slack_user_id(event.user_id.to_string())
                    .text(text.to_string())
                    .download_url(url.to_string())
                    .build();

                put_task(
                    &context.dynamodb_client,
                    Task::builder().kind(download_task.into()).build(),
                )
                .await?;

                Ok("Got it, Will let you know when knowledge ready!\n*You can now revoke the document permission.*".into())
            } else {
                Ok("I cannot learn from that.".into())
            }
        }
        None => Ok("I cannot learn from that.".into()),
    }
}
