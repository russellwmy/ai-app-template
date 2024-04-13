use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::Position;

/// Image.
///
/// ```markdown
/// > | ![a](b)
///     ^^^^^^^
/// ```
#[derive(Debug, Clone, Eq, PartialEq, TypedBuilder, Getters, Serialize, Deserialize)]
pub struct Image {
    // Void.
    /// Positional info.
    position: Option<Position>,
    // Alternative.
    /// Equivalent content for environments that cannot represent the node as
    /// intended.
    alt: String,
    // Resource.
    /// URL to the referenced resource.
    url: String,
    /// Advisory info for the resource, such as something that would be
    /// appropriate for a tooltip.
    title: Option<String>,
}
