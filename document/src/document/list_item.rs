use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{Node, Position};

/// List item.
///
/// ```markdown
/// > | * a
///     ^^^
/// ```
#[derive(Debug, Clone, Eq, PartialEq, TypedBuilder, Getters, Serialize, Deserialize)]
pub struct ListItem {
    // Parent.
    /// Content model.
    children: Vec<Node>,
    /// Positional info.
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<Position>,
    // Extra.
    /// The item contains two or more children separated by a blank line
    /// (when `true`), or not (when `false`).
    spread: bool,
    /// GFM: whether the item is done (when `true`), not done (when `false`),
    /// or indeterminate or not applicable (`None`).
    checked: Option<bool>,
}
