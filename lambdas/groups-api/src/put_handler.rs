use common::extract_sub_from_jwt;
use database::{add_user_to_group, get_group};
use lambda_http::{Request, RequestExt, RequestPayloadExt};
use route_recognizer::Router;
use serde::{Deserialize, Serialize};

use crate::Context;

#[derive(Serialize, Deserialize)]
pub struct AddMemberPayload {
    user_id: String,
}

enum PutRoutes {
    AddMember,
}

pub(crate) async fn process_put_request(
    request: Request,
    context: &Context,
) -> anyhow::Result<serde_json::Value> {
    let token = request.headers().get("Authorization").unwrap().to_str()?;
    let user_id = extract_sub_from_jwt(token)?;
    let mut router = Router::new();
    router.add("/groups/:id", PutRoutes::AddMember);

    let routing = router.recognize(&request.raw_http_path());
    match routing {
        Ok(routing) => {
            let id = routing.params().find("id");
            match id {
                Some(id) => match routing.handler() {
                    PutRoutes::AddMember => {
                        let payload = request.payload::<AddMemberPayload>()?;
                        match payload {
                            Some(payload) => {
                                let group = get_group(&context.dynamodb_client, id).await?;
                                if group.owner_ids().contains(&user_id) {
                                    add_user_to_group(
                                        &context.dynamodb_client,
                                        id,
                                        &payload.user_id,
                                    )
                                    .await?;
                                    Ok(serde_json::json!({}))
                                } else {
                                    Err(anyhow::anyhow!("permission denied"))
                                }
                            }
                            None => Err(anyhow::anyhow!("missing payload")),
                        }
                    }
                },
                None => Err(anyhow::anyhow!("missing collection id")),
            }
        }
        Err(_) => Err(anyhow::anyhow!("Not found")),
    }
}
