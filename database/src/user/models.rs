use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct User {
    pub id: String,
    pub email: String,
    #[builder(default = chrono::Utc::now().timestamp())]
    pub creation_time: i64,
    #[builder(default = "active".to_string())]
    pub status: String,
}
