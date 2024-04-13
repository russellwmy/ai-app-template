use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::TaskKind;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Getters)]
#[serde(rename_all = "snake_case")]
pub struct Task {
    #[builder(default = common::generate_id())]
    id: String,
    #[builder(default = chrono::Utc::now().timestamp())]
    creation_time: i64,
    #[builder(default = chrono::Utc::now().timestamp() + 30)]
    expiration_time: i64,
    #[serde(flatten)]
    kind: TaskKind,
}
