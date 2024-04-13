use std::collections::HashMap;

use database::{
    get_slack_team,
    put_organization,
    put_slack_team,
    put_slack_team_owner,
    update_slack_oauth_data,
    Organization,
    SlackTeam,
};
use lambda_http::{Body, Request, RequestExt};
use route_recognizer::Router;
use slack::utils::get_slack_access_token_by_code;
use tracing::info;

use crate::Context;

enum GetRoutes {
    OAuth,
}

pub(crate) async fn process_get_request(
    request: Request,
    context: &Context,
) -> anyhow::Result<Body> {
    let mut router = Router::new();
    router.add("/:stage/oauth", GetRoutes::OAuth);
    let routing = router.recognize(&request.raw_http_path());

    match routing {
        Ok(routing) => match routing.handler() {
            GetRoutes::OAuth => {
                let query_params = request.query_string_parameters();
                let code = query_params.first("code");
                match code {
                    Some(code) => {
                        let data =
                            get_slack_access_token_by_code(&context.slack_client, code).await?;
                        let oauth_data = serde_json::to_string(&data)?;
                        let slack_team_id = data.team.id.to_string();
                        let slack_team =
                            get_slack_team(&context.dynamodb_client, &slack_team_id).await;
                        match slack_team {
                            Ok(slack_team) => {
                                info!(
                                    event = "app_installed",
                                    organization_id = slack_team.organization_id()
                                );
                                update_slack_oauth_data(
                                    &context.dynamodb_client,
                                    &slack_team.team_id(),
                                    oauth_data,
                                )
                                .await?;
                            }
                            Err(_) => {
                                let organization_id = common::generate_id();

                                let slack_team = SlackTeam::builder()
                                    .team_id(slack_team_id)
                                    .oauth_data(oauth_data)
                                    .organization_id(organization_id.to_owned())
                                    .user_map(HashMap::new())
                                    .channel_map(HashMap::new())
                                    .build();
                                put_slack_team(&context.dynamodb_client, slack_team.to_owned())
                                    .await?;
                                let slack_owner =
                                    put_slack_team_owner(&context.dynamodb_client, slack_team)
                                        .await?;

                                let organization = Organization::builder()
                                    .id(organization_id.to_owned())
                                    .admins(vec![])
                                    .users(vec![])
                                    .creator_id(slack_owner.id)
                                    .build();
                                put_organization(&context.dynamodb_client, organization).await?;

                                info!(event = "app_installed", organization_id = organization_id);
                            }
                        }
                        Ok("Thank you for using us.".into())
                    }
                    None => Err(anyhow::anyhow!("missing auth code.")),
                }
            }
        },
        Err(_) => Err(anyhow::anyhow!("Not found")),
    }
}
