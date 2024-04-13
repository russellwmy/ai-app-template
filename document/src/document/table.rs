use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{Node, Position};

/// GFM: table.
///
/// ```markdown
/// > | | a |
///     ^^^^^
/// > | | - |
///     ^^^^^
/// ```
#[derive(Debug, Clone, Eq, PartialEq, TypedBuilder, Getters, Serialize, Deserialize)]
pub struct Table {
    // Parent.
    /// Content model.
    children: Vec<Node>,
    /// Positional info.
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<Position>,
    // Extra.
    // Represents how cells in columns are aligned.
    //  align: Vec<AlignKind>,
}
