use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Getters)]
#[serde(rename_all = "snake_case")]
pub struct IndexingTask {
    document_id: String,
    group_id: String,
    creator_id: String,
    filename: String,
    file_key: String,
    #[builder(default = None)]
    external_link: Option<String>,
    #[builder(default = None)]
    callback_url: Option<String>,
}
