mod context;
mod embedding;
mod indexer;
mod indexing_meta;

pub mod graph;
pub mod math;
pub mod utils;

pub use context::Context;
pub use embedding::{EmbeddingModel, MiniLMEmbeddingModel, ModelId, OpenAIAdaV2EmbeddingModel};
pub use indexer::Indexer;
pub use indexing_meta::IndexingMeta;

type Result<T> = anyhow::Result<T>;
