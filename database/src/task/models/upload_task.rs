use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Getters)]
#[serde(rename_all = "snake_case")]
pub struct UploadTask {
    document_id: String,
    group_id: String,
    user_id: String,
    filename: String,
    #[builder(default = None)]
    callback_url: Option<String>,
}
