use std::path::PathBuf;

use aws_config::BehaviorVersion;
use aws_lambda_events::event::s3::S3Event;
use database::{delete_task, get_task, put_document, put_task, Document, IndexingTask, Task};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

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
    run(service_fn(move |event: LambdaEvent<S3Event>| async move {
        Ok::<(), Error>(process_request(event, context_ref).await?)
    }))
    .await?;
    Ok(())
}

async fn process_request(event: LambdaEvent<S3Event>, context: &Context) -> Result<(), Error> {
    let s3_event = event.payload;
    if let Some(record) = s3_event.records.first() {
        let bucket_name = record.s3.bucket.name.clone().unwrap();
        let file_key = record.s3.object.key.clone().unwrap();
        let task = get_task(&context.dynamodb_client, &file_key).await?;
        let task_id = task.id().to_string();

        match task.kind() {
            database::TaskKind::UploadTask(task) => {
                let group_id = task.group_id();
                let document_bucket_name = common::vars::get_app_document_bucket()?;
                let document_id = task.document_id();
                let mut new_file_key_path = PathBuf::from(&document_id);
                new_file_key_path.push(task.filename().to_string());
                let new_file_key = new_file_key_path.to_str().unwrap();

                // Copy file
                s3_helper::copy_object(
                    &context.s3_client,
                    &bucket_name,
                    &file_key,
                    &document_bucket_name,
                    &new_file_key,
                )
                .await?;
                s3_helper::delete_object(&context.s3_client, &bucket_name, &file_key).await?;

                let title = common::extract_name_from_filename(task.filename());
                let document = Document::builder()
                    .id(document_id.to_string())
                    .title(title)
                    .group_id(group_id.to_string().to_string())
                    .creator_id(task.user_id().to_string())
                    .filename(task.filename().to_string())
                    .storage_key(document_id.to_string())
                    .index_state("indexing".to_string())
                    .build();
                put_document(&context.dynamodb_client, document).await?;

                let indexing_task = IndexingTask::builder()
                    .document_id(document_id.to_string())
                    .filename(task.filename().to_string())
                    .file_key(new_file_key.to_string())
                    .group_id(task.group_id().to_string())
                    .creator_id(task.user_id().to_string())
                    .callback_url(task.callback_url().to_owned())
                    .build();

                put_task(
                    &context.dynamodb_client,
                    Task::builder().kind(indexing_task.into()).build(),
                )
                .await?;
                delete_task(&context.dynamodb_client, &task_id).await?;
            }
            _ => {}
        }
    }

    Ok(())
}
