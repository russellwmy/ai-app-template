use common::extract_sub_from_jwt;
use database::{
    check_collection_permission,
    delete_collection,
    get_collection,
    update_collection_documents,
};
use lambda_http::{Request, RequestExt};
use route_recognizer::Router;
use tracing::info;

use crate::Context;

enum DeleteRoutes {
    DeleteCollection,
    RemoveDocument,
}

pub(crate) async fn process_delete_request(
    request: Request,
    context: &Context,
) -> anyhow::Result<serde_json::Value> {
    let token = request.headers().get("Authorization").unwrap().to_str()?;
    let user_id = extract_sub_from_jwt(token)?;
    let mut router = Router::new();
    router.add("/collections/:id", DeleteRoutes::DeleteCollection);
    router.add(
        "/collections/:id/documents/:document_id",
        DeleteRoutes::RemoveDocument,
    );

    let routing = router.recognize(&request.raw_http_path());

    match routing {
        Ok(routing) => match routing.handler() {
            DeleteRoutes::DeleteCollection => {
                let id = routing.params().find("id");
                match id {
                    Some(id) => {
                        info!("delete collection: {}", id);
                        let collection = get_collection(&context.dynamodb_client, id).await?;
                        check_collection_permission(
                            &context.dynamodb_client,
                            &collection,
                            &user_id,
                        )
                        .await?;
                        delete_collection(&context.dynamodb_client, id).await?;
                        Ok(serde_json::json!({ "id": id }))
                    }
                    None => Err(anyhow::anyhow!("missing collection id")),
                }
            }
            DeleteRoutes::RemoveDocument => {
                let id = routing.params().find("id");
                let document_id = routing.params().find("document_id");
                match id {
                    Some(id) => match document_id {
                        Some(document_id) => {
                            info!("remove document:{} collection: {}", document_id, id);
                            let collection = get_collection(&context.dynamodb_client, id).await?;
                            check_collection_permission(
                                &context.dynamodb_client,
                                &collection,
                                &user_id,
                            )
                            .await?;
                            let old_document_ids = collection.document_ids().to_vec();
                            let new_documents = old_document_ids
                                .into_iter()
                                .filter(|d| d != document_id)
                                .collect();
                            update_collection_documents(
                                &context.dynamodb_client,
                                id,
                                new_documents,
                            )
                            .await?;
                            Ok(serde_json::json!({ "id": id }))
                        }
                        None => Err(anyhow::anyhow!("missing document id")),
                    },
                    None => Err(anyhow::anyhow!("missing collection id")),
                }
            }
        },
        Err(_) => Err(anyhow::anyhow!("Not found")),
    }
}
