use common::extract_sub_from_jwt;
use database::{check_collection_permission, get_collection, get_document};
use lambda_http::{Request, RequestExt};
use route_recognizer::Router;

use crate::Context;

enum GetRoutes {
    GetCollection,
    GetCollectionDocuments,
}

pub(crate) async fn process_get_request(
    request: Request,
    context: &Context,
) -> anyhow::Result<serde_json::Value> {
    let token = request.headers().get("Authorization").unwrap().to_str()?;
    let user_id = extract_sub_from_jwt(token)?;
    let mut router = Router::new();
    router.add("/collections/:id", GetRoutes::GetCollection);
    router.add(
        "/collections/:id/documents",
        GetRoutes::GetCollectionDocuments,
    );
    let routing = router.recognize(&request.raw_http_path());

    match routing {
        Ok(routing) => match routing.handler() {
            GetRoutes::GetCollection => {
                let id = routing.params().find("id");
                match id {
                    Some(id) => {
                        let collection = get_collection(&context.dynamodb_client, id).await?;
                        check_collection_permission(
                            &context.dynamodb_client,
                            &collection,
                            &user_id,
                        )
                        .await?;
                        Ok(serde_json::to_value(collection)?)
                    }
                    None => Err(anyhow::anyhow!("missing collection id")),
                }
            }
            GetRoutes::GetCollectionDocuments => {
                let id = routing.params().find("id");
                match id {
                    Some(id) => {
                        let collection = get_collection(&context.dynamodb_client, id).await?;
                        check_collection_permission(
                            &context.dynamodb_client,
                            &collection,
                            &user_id,
                        )
                        .await?;
                        let document_ids = collection.document_ids();

                        let mut result = vec![];
                        for document_id in document_ids {
                            if let Ok(document) =
                                get_document(&context.dynamodb_client, document_id).await
                            {
                                result.push(document);
                            }
                        }

                        Ok(serde_json::to_value(result)?)
                    }
                    None => Err(anyhow::anyhow!("missing collection id")),
                }
            }
        },
        Err(_) => Err(anyhow::anyhow!("Not found")),
    }
}
