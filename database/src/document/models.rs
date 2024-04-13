use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct Document {
    #[builder(default = common::generate_id())]
    id: String,
    title: String,
    filename: String,
    storage_key: String,
    index_state: String,
    group_id: String,
    creator_id: String,
    #[builder(default = 0o770)]
    permission: u32, // unix access mode
    #[builder(default = chrono::Utc::now().timestamp())]
    creation_time: i64,
    #[builder(default = "active".to_string())]
    status: String,
}
