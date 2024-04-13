use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct Collection {
    #[builder(default = common::generate_id())]
    id: String,
    name: String,
    document_ids: Vec<String>,
    group_id: String,
    creator_id: String,
    #[builder(default = 0o770)]
    permission: u32, // unix access mode
    #[builder(default = chrono::Utc::now().timestamp())]
    creation_time: i64,
    #[builder(default = "active".to_string())]
    status: String,
}
