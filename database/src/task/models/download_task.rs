use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Getters)]
#[serde(rename_all = "snake_case")]
pub struct DownloadTask {
    collection_id: Option<String>,
    group_id: String,
    creator_id: String,
    filename: Option<String>,
    download_url: String,
    #[builder(default = None)]
    callback_url: Option<String>,
}
