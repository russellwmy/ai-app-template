use common::extract_sub_from_jwt;
use database::{
    check_document_permission,
    delete_document,
    get_collections_by_group_id,
    get_document,
    remove_document_to_collection,
};
use lambda_http::{Request, RequestExt};
use route_recognizer::Router;

use crate::Context;

enum DeleteRoutes {
    DeleteDocument,
}

pub(crate) async fn process_delete_request(
    request: Request,
    context: &Context,
) -> anyhow::Result<serde_json::Value> {
    let token = request.headers().get("Authorization").unwrap().to_str()?;
    let user_id = extract_sub_from_jwt(token)?;
    let mut router = Router::new();
    router.add("/documents/:id", DeleteRoutes::DeleteDocument);
    let routing = router.recognize(&request.raw_http_path());

    match routing {
        Ok(routing) => match routing.handler() {
            DeleteRoutes::DeleteDocument => {
                let id = routing.params().find("id");
                match id {
                    Some(id) => {
                        let document = get_document(&context.dynamodb_client, id).await?;
                        check_document_permission(&context.dynamodb_client, &document, &user_id)
                            .await?;

                        let group_id = document.group_id();
                        // Clean up collections
                        let collections =
                            get_collections_by_group_id(&context.dynamodb_client, group_id).await?;
                        for collection in collections {
                            remove_document_to_collection(
                                &context.dynamodb_client,
                                collection.id(),
                                document.id(),
                            )
                            .await?;
                        }
                        // Start delete documents
                        let bucket_name = common::vars::get_app_uploads_bucket()?;
                        s3_helper::delete_object(
                            &context.s3_client,
                            &bucket_name,
                            document.storage_key(),
                        )
                        .await?;
                        delete_document(&context.dynamodb_client, id).await?;
                        // End delete documents
                        Ok(serde_json::json!({ "id": id }))
                    }
                    None => Err(anyhow::anyhow!("missing collection id")),
                }
            }
        },
        Err(_) => Err(anyhow::anyhow!("Not found")),
    }
}
