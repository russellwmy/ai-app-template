use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct Context {
    score: f32,
    raw_data: String,
    data: String,
    reference: Option<String>,
}
