pub mod chunking;
pub mod collector;
pub mod document;
pub mod docx;
pub mod mime;
pub mod pdf;
pub mod utils;

type Result<T> = std::result::Result<T, anyhow::Error>;
