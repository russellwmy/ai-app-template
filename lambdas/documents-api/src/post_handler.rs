use std::time::Duration;

use chrono::Utc;
use common::extract_sub_from_jwt;
use database::{put_task, DownloadTask, Task, UploadTask};
use lambda_http::{Request, RequestExt, RequestPayloadExt};
use route_recognizer::Router;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::Context;

enum PostRoutes {
    CreateUploadUrl,
    CollectFromUrl,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUploadUrlPayload {
    filename: String,
    group_id: String,
    callback_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CollectFromUrlPayload {
    url: String,
    filename: String,
    group_id: String,
    callback_url: Option<String>,
}

pub(crate) async fn process_post_request(
    request: Request,
    context: &Context,
) -> anyhow::Result<serde_json::Value> {
    info!("called process_post_request: {:?}", request);
    let token = request.headers().get("Authorization").unwrap().to_str()?;
    let user_id = extract_sub_from_jwt(token)?;
    let mut router = Router::new();
    router.add("/documents/upload-url", PostRoutes::CreateUploadUrl);
    router.add("/documents/collect", PostRoutes::CollectFromUrl);
    let routing = router.recognize(&request.raw_http_path());

    match routing {
        Ok(routing) => match routing.handler() {
            PostRoutes::CreateUploadUrl => {
                let payload = request.payload::<CreateUploadUrlPayload>()?;
                match payload {
                    Some(payload) => {
                        let expiration = 5 * 60;
                        let bucket_name = common::vars::get_app_uploads_bucket()?;
                        let file_key = common::generate_id();
                        let upload_url = s3_helper::get_presigned_upload_url(
                            &context.s3_client,
                            &bucket_name,
                            &file_key,
                            Duration::from_secs(expiration),
                        )
                        .await?;

                        let document_id = common::generate_id();
                        let upload_task = UploadTask::builder()
                            .document_id(document_id.to_string())
                            .group_id(payload.group_id)
                            .user_id(user_id.to_string())
                            .filename(payload.filename.to_string())
                            .callback_url(payload.callback_url)
                            .build();

                        put_task(
                            &context.dynamodb_client,
                            Task::builder()
                                .id(file_key.to_string()) // for retrieve file
                                .kind(upload_task.into())
                                .expiration_time(Utc::now().timestamp() + expiration as i64)
                                .build(),
                        )
                        .await?;
                        Ok(serde_json::json!({
                            "document_id":document_id.to_string(),
                            "upload_url": upload_url.to_string(),
                        }))
                    }
                    None => Err(anyhow::anyhow!("missing payload")),
                }
            }
            PostRoutes::CollectFromUrl => {
                let payload = request.payload::<CollectFromUrlPayload>()?;
                match payload {
                    Some(payload) => {
                        let download_task = DownloadTask::builder()
                            .group_id(payload.group_id)
                            .collection_id(None)
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
                    None => Err(anyhow::anyhow!("missing payload")),
                }
            }
        },
        Err(_) => Err(anyhow::anyhow!("Not found")),
    }
}
