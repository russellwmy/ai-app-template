use std::error::Error;

use super::{Message, OpenAIComposer};

#[derive(Debug)]
pub enum Composer {
    OpenAIComposer(OpenAIComposer),
}

impl Composer {
    pub async fn compose(
        &self,
        context: &str,
        query: &str,
    ) -> Result<(String, Message), Box<dyn Error>> {
        match self {
            Composer::OpenAIComposer(composer) => composer.compose(context, query).await,
        }
    }
}

impl From<OpenAIComposer> for Composer {
    fn from(composer: OpenAIComposer) -> Self {
        Composer::OpenAIComposer(composer)
    }
}
