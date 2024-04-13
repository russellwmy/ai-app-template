use common::extract_sub_from_jwt;
use database::{
    check_group_member, get_collections_by_group_id, get_documents_by_group_id, get_group,
};
use lambda_http::{Request, RequestExt};
use route_recognizer::Router;

use crate::Context;

enum GetRoutes {
    GetGroup,
    GetGroupDocuments,
    GetGroupCollections,
}

pub(crate) async fn process_get_request(
    request: Request,
    context: &Context,
) -> anyhow::Result<serde_json::Value> {
    let token = request.headers().get("Authorization").unwrap().to_str()?;
    let user_id = extract_sub_from_jwt(token)?;
    let mut router = Router::new();

    router.add("/groups/:id", GetRoutes::GetGroup);
    router.add("/groups/:id/documents", GetRoutes::GetGroupDocuments);
    router.add("/groups/:id/collections", GetRoutes::GetGroupCollections);
    let routing = router.recognize(&request.raw_http_path());

    match routing {
        Ok(routing) => match routing.handler() {
            GetRoutes::GetGroup => {
                let id = routing.params().find("id");
                match id {
                    Some(id) => {
                        let group = get_group(&context.dynamodb_client, id).await?;
                        check_group_member(&context.dynamodb_client, id, &user_id).await?;

                        Ok(serde_json::to_value(group)?)
                    }
                    None => Err(anyhow::anyhow!("missing group id")),
                }
            }
            GetRoutes::GetGroupDocuments => {
                let id = routing.params().find("id");
                match id {
                    Some(id) => {
                        let group = get_group(&context.dynamodb_client, id).await?;
                        let result =
                            get_documents_by_group_id(&context.dynamodb_client, id).await?;

                        if group.user_ids().contains(&user_id) {
                            Ok(serde_json::to_value(result)?)
                        } else {
                            Err(anyhow::anyhow!("permission denied"))
                        }
                    }
                    None => Err(anyhow::anyhow!("missing group id")),
                }
            }
            GetRoutes::GetGroupCollections => {
                let id = routing.params().find("id");
                match id {
                    Some(id) => {
                        let group = get_group(&context.dynamodb_client, id).await?;
                        let result =
                            get_collections_by_group_id(&context.dynamodb_client, id).await?;
                        if group.user_ids().contains(&user_id) {
                            Ok(serde_json::to_value(result)?)
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
