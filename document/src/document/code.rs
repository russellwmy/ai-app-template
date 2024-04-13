use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::Position;
/// Code (flow).
///
/// ```markdown
/// > | ~~~
///     ^^^
/// > | a
///     ^
/// > | ~~~
///     ^^^
/// ```
#[derive(Debug, Clone, Eq, PartialEq, TypedBuilder, Getters, Serialize, Deserialize)]
pub struct Code {
    // Text.
    /// Content model.
    value: String,
    /// Positional info.
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<Position>,
    // Extra.
    /// The language of computer code being marked up.
    lang: Option<String>,
    /// Custom info relating to the node.
    meta: Option<String>,
}
