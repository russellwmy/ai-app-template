use serde::{Deserialize, Serialize};

mod constants;
mod utils;

pub use utils::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MIMEType {
    extension: String,
    description: String,
    mime_type: String,
}
