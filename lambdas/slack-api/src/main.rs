use lambda_http::{
    http::{Method, StatusCode},
    run,
    service_fn,
    Body,
    Error,
    Request,
    RequestExt,
    Response,
};

mod get_handler;
mod post_handler;

use get_handler::process_get_request;
use post_handler::process_post_request;
use slack_morphism::{
    prelude::{SlackClientHyperConnector, SlackHyperClient},
    SlackClient,
};
use tracing::info;

struct Context {
    dynamodb_client: aws_sdk_dynamodb::Client,
    slack_client: SlackHyperClient,
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
    let slack_client = SlackClient::new(SlackClientHyperConnector::new());
    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);
    let context = Context {
        dynamodb_client,
        slack_client,
    };
    let context_ref = &context;

    run(service_fn(move |req: Request| async move {
        process_request(req, context_ref).await
    }))
    .await?;
    Ok(())
}

async fn process_request(request: Request, context: &Context) -> Result<Response<Body>, Error> {
    let result = match request.method().to_owned() {
        Method::GET => process_get_request(request, context).await,
        Method::POST => process_post_request(request, context).await,
        _ => Err(anyhow::anyhow!("Unhandled method")),
    };

    let response = match result {
        Ok(data) => Response::builder()
            .status(StatusCode::OK)
            .body(data.into())
            .map_err(Box::new)?,
        Err(err) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(err.to_string().into())
            .map_err(Box::new)?,
    };
    Ok(response)
}
