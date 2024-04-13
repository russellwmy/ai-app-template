use std::path::PathBuf;

use aws_config::BehaviorVersion;
use aws_lambda_events::dynamodb::Event;
use aws_sdk_s3::primitives::ByteStream;
use database::{
    add_document_to_collection, delete_task, from_item, put_document, put_task, Document,
    DownloadTask, IndexingTask, Task, TaskKind,
};
use document::collector::{is_google_docs_url, parse_google_docs_url, FileCollector};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use tracing::{error, info};

struct Context {
    s3_client: aws_sdk_s3::Client,
    dynamodb_client: aws_sdk_dynamodb::Client,
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

    //Get config from environment.
    let config = aws_config::load_defaults(BehaviorVersion::v2023_11_09()).await;
    let s3_client = aws_sdk_s3::Client::new(&config);
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);
    let context = Context {
        s3_client,
        dynamodb_client,
    };
    let context_ref = &context;
    run(service_fn(move |event: LambdaEvent<Event>| async move {
        Ok::<(), Error>(process_request(event, context_ref).await?)
    }))
    .await?;
    Ok(())
}

async fn process_request(event: LambdaEvent<Event>, context: &Context) -> Result<(), Error> {
    if let Some(record) = event.payload.records.last() {
        let task: Task = from_item(record.change.new_image.clone())?;
        let task_id = task.id().to_string();
        match task.kind() {
            TaskKind::DownloadTask(task) => {
                let result = process_task(task, context).await;
                match result {
                    Ok(_) => delete_task(&context.dynamodb_client, &task_id).await?,
                    Err(e) => {
                        error!("{}", e.to_string());
                        delete_task(&context.dynamodb_client, &task_id).await?;
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}

async fn process_task(task: &DownloadTask, context: &Context) -> Result<(), Error> {
    info!("process download task");
    let download_url = common::clean_url(task.download_url());
    let default_filename = task.filename().clone().unwrap_or("data.docx".to_string());

    // Collect the document
    common::validate_url(&download_url)?;
    let download_url = match is_google_docs_url(&download_url) {
        true => parse_google_docs_url(&download_url)?,
        false => download_url.to_string(),
    };

    info!("process download url: {}", download_url);
    let collector = FileCollector::new();
    let collect_file = collector.collect(&download_url).await?;
    let filename = collect_file
        .filename
        .unwrap_or(default_filename.to_string());
    let bucket_name = common::vars::get_app_document_bucket()?;
    info!("downloaded file: {}", filename);

    let group_id = task.group_id();
    let collection_id = task.collection_id();
    let user_id = task.creator_id();
    let document_id = common::generate_id();

    let mut key = PathBuf::from(&document_id);

    key.push(&filename);
    let file_key = key.to_str().unwrap();
    s3_helper::upload_object_with_content(
        &context.s3_client,
        &bucket_name,
        &file_key,
        ByteStream::from(collect_file.content),
    )
    .await?;

    let title = common::extract_name_from_filename(&filename);
    let doc = Document::builder()
        .id(document_id.to_string())
        .title(title.to_string())
        .filename(filename.to_string())
        .creator_id(user_id.to_string())
        .group_id(group_id.to_string())
        .storage_key(document_id.to_string())
        .index_state("indexing".to_string())
        .build();
    put_document(&context.dynamodb_client, doc).await?;
    info!("created document: {}", document_id);

    if let Some(collection_id) = collection_id {
        add_document_to_collection(&context.dynamodb_client, &collection_id, &document_id).await?;
        info!(
            "added document: {} to collection: {}",
            document_id, collection_id
        );
    }

    let indexing_task = IndexingTask::builder()
        .group_id(group_id.to_string())
        .creator_id(user_id.to_string())
        .document_id(document_id.to_string())
        .callback_url(task.callback_url().to_owned())
        .filename(filename.to_string())
        .file_key(file_key.to_string())
        .external_link(Some(task.download_url().to_string()))
        .build();

    put_task(
        &context.dynamodb_client,
        Task::builder().kind(indexing_task.into()).build(),
    )
    .await?;
    Ok(())
}
