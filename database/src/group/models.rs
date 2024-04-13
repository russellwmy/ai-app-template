use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct Group {
    #[builder(default = common::generate_id())]
    id: String,
    name: Option<String>,
    owner_ids: Vec<String>,
    user_ids: Vec<String>,
    creator_id: String,
    #[builder(default = chrono::Utc::now().timestamp())]
    creation_time: i64,
    #[builder(default = "active".to_string())]
    status: String,
}
