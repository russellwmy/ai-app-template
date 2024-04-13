use std::{path::Path, time::Duration};

use aws_sdk_s3::{
    error::SdkError,
    operation::{
        copy_object::{CopyObjectError, CopyObjectOutput},
        create_bucket::{CreateBucketError, CreateBucketOutput},
        get_object::{GetObjectError, GetObjectOutput},
        list_objects_v2::ListObjectsV2Output,
        put_object::{PutObjectError, PutObjectOutput},
    },
    presigning::PresigningConfig,
    primitives::ByteStream,
    types::{BucketLocationConstraint, CreateBucketConfiguration, Delete, ObjectIdentifier},
    Client,
};
mod error;
use error::Error;

pub async fn delete_bucket(client: &Client, bucket_name: &str) -> Result<(), Error> {
    client.delete_bucket().bucket(bucket_name).send().await?;
    println!("Bucket deleted");
    Ok(())
}

pub async fn delete_objects(client: &Client, bucket_name: &str) -> Result<Vec<String>, Error> {
    let objects = client.list_objects_v2().bucket(bucket_name).send().await?;

    let mut delete_objects: Vec<ObjectIdentifier> = vec![];
    for obj in objects.contents() {
        let obj_id = ObjectIdentifier::builder()
            .set_key(Some(obj.key().unwrap().to_string()))
            .build();
        match obj_id {
            Ok(obj_id) => delete_objects.push(obj_id),
            Err(_) => (),
        }
    }

    let return_keys = delete_objects.iter().map(|o| o.key().to_string()).collect();
    let delete_command = Delete::builder()
        .set_objects(Some(delete_objects))
        .build()?;
    client
        .delete_objects()
        .bucket(bucket_name)
        .delete(delete_command)
        .send()
        .await?;

    let objects: ListObjectsV2Output = client.list_objects_v2().bucket(bucket_name).send().await?;

    eprintln!("{objects:?}");

    match objects.key_count {
        Some(_) => Ok(return_keys),
        _ => Err(Error::unhandled(
            "There were still objects left in the bucket.",
        )),
    }
}

pub async fn list_objects(
    client: &Client,
    bucket_name: &str,
    prefix: &str,
) -> Result<Vec<String>, Error> {
    let objects = client
        .list_objects_v2()
        .bucket(bucket_name)
        .prefix(prefix)
        .send()
        .await?;
    let mut result = vec![];
    for obj in objects.contents() {
        result.push(obj.key().unwrap().to_string());
    }

    Ok(result)
}

pub async fn copy_object(
    client: &Client,
    source_bucket_name: &str,
    source_key: &str,
    target_bucket_name: &str,
    target_key: &str,
) -> Result<CopyObjectOutput, SdkError<CopyObjectError>> {
    let mut source_bucket_and_object: String = "".to_owned();
    source_bucket_and_object.push_str(source_bucket_name);
    source_bucket_and_object.push('/');
    source_bucket_and_object.push_str(source_key);

    client
        .copy_object()
        .copy_source(source_bucket_and_object)
        .bucket(target_bucket_name)
        .key(target_key)
        .send()
        .await
}

pub async fn download_object(
    client: &Client,
    bucket_name: &str,
    key: &str,
) -> Result<GetObjectOutput, SdkError<GetObjectError>> {
    client
        .get_object()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await
}

pub async fn get_presigned_upload_url(
    client: &Client,
    bucket_name: &str,
    key: &str,
    expires_in: Duration,
) -> Result<String, SdkError<PutObjectError>> {
    let presigning_config = PresigningConfig::builder()
        .expires_in(expires_in)
        .build()
        .unwrap();

    let presigned_request = client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .presigned(presigning_config)
        .await?;
    Ok(presigned_request.uri().to_owned())
}

pub async fn upload_object(
    client: &Client,
    bucket_name: &str,
    file_name: &str,
    key: &str,
) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
    let body = ByteStream::from_path(Path::new(file_name)).await;
    client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body.unwrap())
        .send()
        .await
}

pub async fn upload_object_with_content(
    client: &Client,
    bucket_name: &str,
    key: &str,
    body: ByteStream,
) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
    client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body)
        .send()
        .await
}

pub async fn create_bucket(
    client: &Client,
    bucket_name: &str,
    region: &str,
) -> Result<CreateBucketOutput, SdkError<CreateBucketError>> {
    let constraint = BucketLocationConstraint::from(region);
    let cfg = CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();
    client
        .create_bucket()
        .create_bucket_configuration(cfg)
        .bucket(bucket_name)
        .send()
        .await
}

pub async fn delete_object(client: &Client, bucket_name: &str, key: &str) -> Result<(), Error> {
    client
        .delete_object()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await?;

    Ok(())
}
