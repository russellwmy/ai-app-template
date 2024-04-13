use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{Node, Position};

/// List.
///
/// ```markdown
/// > | * a
///     ^^^
/// ```
#[derive(Debug, Clone, Eq, PartialEq, TypedBuilder, Getters, Serialize, Deserialize)]
pub struct List {
    // Parent.
    /// Content model.
    children: Vec<Node>,
    /// Positional info.
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<Position>,
    // Extra.
    /// Ordered (`true`) or unordered (`false`).
    ordered: bool,
    /// Starting number of the list.
    /// `None` when unordered.
    start: Option<u32>,
    /// One or more of its children are separated with a blank line from its
    /// siblings (when `true`), or not (when `false`).
    spread: bool,
}
