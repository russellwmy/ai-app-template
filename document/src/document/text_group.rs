use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{Node, Position};

#[derive(Debug, Clone, Eq, PartialEq, TypedBuilder, Getters, Serialize, Deserialize)]
pub struct TextGroup {
    // Parent.
    /// Content model.
    children: Vec<Node>,
    /// Positional info.
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<Position>,
}
