use anyhow::Result;
use reqwest::Client;

use crate::mime::is_supported_mime_type;

use super::CollectedFile;

pub struct FileCollector {
    client: Client,
}

impl FileCollector {
    pub fn new() -> Self {
        let client = Client::new();
        Self { client }
    }

    pub async fn collect(&self, url: &str) -> Result<CollectedFile> {
        let response = self.client.get(url).send().await?;
        let headers = response.headers();
        let filename = match headers.get("content-disposition") {
            Some(content_disposition) => common::extract_filename_from_content_disposition(
                content_disposition.to_str().unwrap(),
            ),
            None => None,
        };
        let content_type = match headers.get("content-type") {
            Some(value) => {
                let mime_type = value.to_str().unwrap().to_string();
                if is_supported_mime_type(&mime_type) {
                    Ok(mime_type)
                } else {
                    Err(anyhow::anyhow!("Unsupported document"))
                }
            }
            None => Err(anyhow::anyhow!("Unsupported document")),
        }?;

        let content = response.bytes().await?;

        Ok(CollectedFile {
            filename,
            content_type,
            content: content.to_vec(),
        })
    }
}
