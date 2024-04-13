use common::extract_sub_from_jwt;
use database::{
    check_collection_permission,
    check_document_permission,
    get_collection,
    get_document,
    put_collection,
    put_task,
    update_collection_documents,
    Collection,
    DownloadTask,
    Task,
};
use lambda_http::{Request, RequestExt, RequestPayloadExt};
use route_recognizer::Router;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::Context;

enum PostRoutes {
    CreateCollection,
    AddDocument,
    CollectDocumentFromUrl,
}

#[derive(Serialize, Deserialize)]
pub struct CreateCollectionPayload {
    name: String,
    group_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddDocumentPayload {
    document_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CollectDocumentFromUrlPayload {
    url: String,
    filename: String,
    group_id: String,
    callback_url: Option<String>,
}

pub(crate) async fn process_post_request(
    request: Request,
    context: &Context,
) -> anyhow::Result<serde_json::Value> {
    let token = request.headers().get("Authorization").unwrap().to_str()?;
    let user_id = extract_sub_from_jwt(token)?;
    let mut router = Router::new();
    router.add("/collections", PostRoutes::CreateCollection);
    router.add("/collections/:id/documents", PostRoutes::AddDocument);
    router.add(
        "/collections/:id/documents/collect",
        PostRoutes::CollectDocumentFromUrl,
    );
    let routing = router.recognize(&request.raw_http_path());
    info!("{}", request.raw_http_path());
    match routing {
        Ok(routing) => match routing.handler() {
            PostRoutes::CreateCollection => {
                info!("create collection");
                let payload = request.payload::<CreateCollectionPayload>()?;
                match payload {
                    Some(payload) => {
                        let collection: Collection = Collection::builder()
                            .name(payload.name)
                            .document_ids(vec![])
                            .creator_id(user_id.to_string())
                            .group_id(payload.group_id.to_string())
                            .build();

                        put_collection(&context.dynamodb_client, collection.to_owned()).await?;

                        Ok(serde_json::to_value(collection)?)
                    }
                    None => Err(anyhow::anyhow!("missing payload")),
                }
            }
            PostRoutes::AddDocument => {
                info!("add document");
                let payload = request.payload::<AddDocumentPayload>()?;
                match payload {
                    Some(payload) => {
                        let id = routing.params().find("id");
                        match id {
                            Some(id) => {
                                let document_id = payload.document_id;
                                let collection =
                                    get_collection(&context.dynamodb_client, id).await?;
                                check_collection_permission(
                                    &context.dynamodb_client,
                                    &collection,
                                    &user_id,
                                )
                                .await?;
                                let document =
                                    get_document(&context.dynamodb_client, &document_id).await?;
                                check_document_permission(
                                    &context.dynamodb_client,
                                    &document,
                                    &user_id,
                                )
                                .await?;

                                info!("add document:{} to colleciton: {}", document_id, id);
                                let mut new_documents = collection.document_ids().to_vec();
                                new_documents.push(document_id);
                                update_collection_documents(
                                    &context.dynamodb_client,
                                    id,
                                    new_documents,
                                )
                                .await?;
                                Ok(serde_json::json!({"message": "done"}))
                            }
                            None => Err(anyhow::anyhow!("missing payload")),
                        }
                    }
                    None => Err(anyhow::anyhow!("missing payload")),
                }
            }

            PostRoutes::CollectDocumentFromUrl => {
                info!("collection collect document from url");
                let payload = request.payload::<CollectDocumentFromUrlPayload>()?;
                match payload {
                    Some(payload) => {
                        let id = routing.params().find("id");
                        match id {
                            Some(id) => {
                                let download_task = DownloadTask::builder()
                                    .group_id(payload.group_id)
                                    .collection_id(Some(id.to_string()))
                                    .creator_id(user_id.to_string())
                                    .filename(Some(payload.filename.to_string()))
                                    .download_url(payload.url.to_string())
                                    .callback_url(payload.callback_url)
                                    .build();
                                put_task(
                                    &context.dynamodb_client,
                                    Task::builder().kind(download_task.into()).build(),
                                )
                                .await?;
                                Ok(serde_json::json!({
                                    "download_url": payload.url.to_string(),
                                }))
                            }

                            None => Err(anyhow::anyhow!("missing project id")),
                        }
                    }
                    None => Err(anyhow::anyhow!("missing payload")),
                }
            }
        },
        Err(_) => Err(anyhow::anyhow!("Not found")),
    }
}
