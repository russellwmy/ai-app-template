use common::extract_sub_from_jwt;
use database::{delete_group, get_group};
use lambda_http::{Request, RequestExt};
use route_recognizer::Router;

use crate::Context;

enum DeleteRoutes {
    DeleteOrganization,
}

pub(crate) async fn process_delete_request(
    request: Request,
    context: &Context,
) -> anyhow::Result<serde_json::Value> {
    let token = request.headers().get("Authorization").unwrap().to_str()?;
    let user_id = extract_sub_from_jwt(token)?;
    let mut router = Router::new();
    router.add("/groups/:id", DeleteRoutes::DeleteOrganization);

    let routing = router.recognize(&request.raw_http_path());

    match routing {
        Ok(routing) => match routing.handler() {
            DeleteRoutes::DeleteOrganization => {
                let id = routing.params().find("id");
                match id {
                    Some(id) => {
                        let group = get_group(&context.dynamodb_client, id).await?;
                        if group.creator_id() == &user_id {
                            delete_group(&context.dynamodb_client, id).await?;
                            Ok(serde_json::json!({ "id": id }))
                        } else {
                            Err(anyhow::anyhow!("permission denied"))
                        }
                    }
                    None => Err(anyhow::anyhow!("missing group id")),
                }
            }
        },
        Err(_) => Err(anyhow::anyhow!("Not found")),
    }
}
