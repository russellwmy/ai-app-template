use common::extract_sub_from_jwt;
use database::{get_group_creator, put_group, Group};
use lambda_http::{Request, RequestExt};
use route_recognizer::Router;
use serde::{Deserialize, Serialize};

use crate::Context;

enum PostRoutes {
    CreateGroup,
}

#[derive(Serialize, Deserialize)]
pub struct CreateGroup {}

pub(crate) async fn process_post_request(
    request: Request,
    context: &Context,
) -> anyhow::Result<serde_json::Value> {
    let token = request.headers().get("Authorization").unwrap().to_str()?;
    let user_id = extract_sub_from_jwt(token)?;
    let mut router = Router::new();
    router.add("/groups", PostRoutes::CreateGroup);
    let routing = router.recognize(&request.raw_http_path());
    match routing {
        Ok(routing) => match routing.handler() {
            PostRoutes::CreateGroup => {
                let group_result = get_group_creator(&context.dynamodb_client, &user_id).await;
                match group_result {
                    Ok(group) => Ok(serde_json::to_value(group)?),
                    Err(_) => {
                        let data = Group::builder()
                            .name(None)
                            .owner_ids(vec![user_id.to_string()])
                            .user_ids(vec![user_id.to_string()])
                            .creator_id(user_id.to_string())
                            .build();

                        put_group(&context.dynamodb_client, data.to_owned()).await?;

                        Ok(serde_json::to_value(data)?)
                    }
                }
            }
            _ => Err(anyhow::anyhow!("Not found")),
        },
        Err(_) => Err(anyhow::anyhow!("Not found")),
    }
}
