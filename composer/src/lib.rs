mod composer;
mod composer_enum;
mod openai;
mod prompt;
pub mod utils;

pub type Message = String;

type Result<T> = anyhow::Result<T>;

pub use composer::Composer;
pub use composer_enum::ComposerEnum;
pub use openai::OpenAIComposer;
