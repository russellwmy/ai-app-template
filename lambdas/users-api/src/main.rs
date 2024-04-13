use aws_config::BehaviorVersion;
use lambda_http::{
    http::{Method, StatusCode},
    run, service_fn, Error, IntoResponse, Request, RequestExt, Response,
};

mod put_handler;

use put_handler::process_put_request;

struct Context {
    dynamodb_client: aws_sdk_dynamodb::Client,
    cognitoidentityprovider_client: aws_sdk_cognitoidentityprovider::Client,
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
    let config = aws_config::load_defaults(BehaviorVersion::v2023_11_09()).await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);
    let cognitoidentityprovider_client = aws_sdk_cognitoidentityprovider::Client::new(&config);
    let context = Context {
        dynamodb_client,
        cognitoidentityprovider_client,
    };
    let context_ref = &context;

    run(service_fn(move |req: Request| async move {
        process_request(req, context_ref).await
    }))
    .await?;
    Ok(())
}

async fn process_request(request: Request, context: &Context) -> Result<impl IntoResponse, Error> {
    let result = match request.method().to_owned() {
        Method::PUT => process_put_request(request, context).await,
        _ => Err(anyhow::anyhow!("Unhandled method")),
    };

    let response = match result {
        Ok(data) => Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&data).unwrap())
            .map_err(Box::new)?,
        Err(err) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("Content-Type", "application/json")
            .body(serde_json::json!({"error": err.to_string()}).to_string())
            .map_err(Box::new)?,
    };
    Ok(response)
}
