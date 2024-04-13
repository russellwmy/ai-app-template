use aws_config::BehaviorVersion;
use common::extract_sub_from_jwt;
use composer::{
    utils::{compose_message_with_context, compose_message_with_graph, get_composer},
    Composer,
};
use database as db;
use db::{put_task, CallbackTask, Task};
use indexer::{
    utils::{get_embedding_model, load_graphs_from_s3, search_graph},
    EmbeddingModel,
};
use lambda_http::{
    http::StatusCode, run, service_fn, Error, IntoResponse, Request, RequestPayloadExt, Response,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::log::info;

struct Context {
    s3_client: aws_sdk_s3::Client,
    dynamodb_client: aws_sdk_dynamodb::Client,
    model: EmbeddingModel,
    composer: Composer,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let resources_path = common::vars::get_app_resources_path()?;
    let config = aws_config::load_defaults(BehaviorVersion::v2023_11_09()).await;
    let s3_client = aws_sdk_s3::Client::new(&config);
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);
    let model = get_embedding_model(&resources_path).await?;
    let composer = get_composer(1800).await?;
    let context = Context {
        s3_client,
        dynamodb_client,
        model,
        composer,
    };
    let context_ref = &context;
    run(service_fn(move |req: Request| async move {
        process_request(req, context_ref).await
    }))
    .await?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct Payload {
    collection_id: String,
    query: String,
    context: Option<String>,
    callback_url: Option<String>,
}

async fn process_request(
    request: Request,
    Context {
        s3_client,
        dynamodb_client,
        model,
        composer,
    }: &Context,
) -> Result<impl IntoResponse, Error> {
    let token = request.headers().get("Authorization").unwrap().to_str()?;
    let user_id = extract_sub_from_jwt(token)?;
    let payload = request.payload::<Payload>()?;

    let result = match payload {
        Some(payload) => {
            let query = payload.query;
            let context = payload.context;

            let result = match context {
                Some(context) => {
                    info!("componse message with context");
                    let message =
                        compose_message_with_context(composer, context.to_string(), &query).await?;

                    json!({ "message": message, "context": context})
                }

                None => {
                    info!("componse message without context");
                    let collection =
                        db::get_collection(&dynamodb_client, &payload.collection_id).await?;
                    let bucket_name = common::vars::get_app_document_bucket()?;
                    let document_keys = collection
                        .document_ids()
                        .iter()
                        .map(|k| k.as_str())
                        .collect();
                    let graphs =
                        load_graphs_from_s3(&s3_client, &bucket_name, document_keys).await?;
                    let nodes = search_graph(graphs, &query, model).await?;
                    let (message, context) =
                        compose_message_with_graph(composer, nodes, &query, 2048).await?;

                    json!({ "message": message, "context": context})
                }
            };

            if let Some(callback_url) = payload.callback_url {
                info!("create callback task");
                let callback_task = CallbackTask::builder()
                    .data(result.clone())
                    .callback_url(callback_url)
                    .caller_id(user_id)
                    .build();
                let task = Task::builder().kind(callback_task.into()).build();
                put_task(dynamodb_client, task).await?;
            }

            Ok(result)
        }
        None => Err(anyhow::anyhow!("missing payload")),
    };

    info!("Result: {:?}", result);

    let response = match result {
        Ok(data) => Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(data.to_string())
            .map_err(Box::new)?,
        Err(err) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("Content-Type", "application/json")
            .body(json!({"error": err.to_string()}).to_string())
            .map_err(Box::new)?,
    };

    Ok(response)
}
