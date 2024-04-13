use common::extract_sub_from_jwt;
use database::{check_collection_permission, get_collection, update_collection_name};
use lambda_http::{Request, RequestExt, RequestPayloadExt};
use route_recognizer::Router;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::Context;

#[derive(Serialize, Deserialize)]
pub struct PutPayload {
    name: String,
}

enum PutRoutes {
    UpdateCollection,
}

pub(crate) async fn process_put_request(
    request: Request,
    context: &Context,
) -> anyhow::Result<serde_json::Value> {
    let token = request.headers().get("Authorization").unwrap().to_str()?;
    let user_id = extract_sub_from_jwt(token)?;
    let mut router = Router::new();
    router.add("/collections/:id", PutRoutes::UpdateCollection);

    let routing = router.recognize(&request.raw_http_path());
    match routing {
        Ok(routing) => {
            let id = routing.params().find("id");
            match id {
                Some(id) => match routing.handler() {
                    PutRoutes::UpdateCollection => {
                        let payload = request.payload::<PutPayload>()?;
                        match payload {
                            Some(payload) => {
                                let collection =
                                    get_collection(&context.dynamodb_client, id).await?;
                                check_collection_permission(
                                    &context.dynamodb_client,
                                    &collection,
                                    &user_id,
                                )
                                .await?;
                                info!(
                                    "call update_collection_name, id:{}, name: {}",
                                    id, payload.name
                                );
                                update_collection_name(&context.dynamodb_client, id, &payload.name)
                                    .await?;
                                Ok(serde_json::json!({ "id": id }))
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
