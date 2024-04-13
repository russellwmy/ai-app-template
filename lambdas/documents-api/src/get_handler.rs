use common::extract_sub_from_jwt;
use database::{check_document_permission, get_document};
use lambda_http::{Request, RequestExt};
use route_recognizer::Router;
use tracing::info;

use crate::Context;

enum GetRoutes {
    GetDocument,
}

pub(crate) async fn process_get_request(
    request: Request,
    context: &Context,
) -> anyhow::Result<serde_json::Value> {
    info!("called process_get_request");
    let token = request.headers().get("Authorization").unwrap().to_str()?;
    let user_id = extract_sub_from_jwt(token)?;
    let mut router = Router::new();
    router.add("/documents/:id", GetRoutes::GetDocument);
    let routing = router.recognize(&request.raw_http_path());

    match routing {
        Ok(routing) => match routing.handler() {
            GetRoutes::GetDocument => {
                let id = routing.params().find("id");
                match id {
                    Some(id) => {
                        let document = get_document(&context.dynamodb_client, id).await?;
                        check_document_permission(&context.dynamodb_client, &document, &user_id)
                            .await?;

                        Ok(serde_json::to_value(document)?)
                    }
                    None => Err(anyhow::anyhow!("missing documents id")),
                }
            }
        },
        Err(_) => Err(anyhow::anyhow!("Not found")),
    }
}
