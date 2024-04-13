#[derive(Debug, Clone)]
pub struct CollectedFile {
    pub filename: Option<String>,
    pub content_type: String,
    pub content: Vec<u8>,
}
