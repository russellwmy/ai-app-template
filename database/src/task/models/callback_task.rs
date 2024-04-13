use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Getters)]
#[serde(rename_all = "snake_case")]
pub struct CallbackTask {
    caller_id: String,
    data: Value,
    callback_url: String,
}
