use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder, Getters, Serialize, Deserialize)]
pub struct DocumentMeta {
    title: String,
    language: Option<String>,
    author: Option<String>,
    creator: Option<String>,
    producer: Option<String>,
    subject: Option<String>,
    description: Option<String>,
    keywords: Option<String>,
    creation_date: Option<i64>,
    modification_date: Option<i64>,
}
