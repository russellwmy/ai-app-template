use std::error::Error;

use async_openai::{
    config::OpenAIConfig,
    types::{CreateChatCompletionRequestArgs, CreateCompletionRequestArgs},
    Client,
};

use super::{prompt, ComposerEnum, Message};

#[derive(Debug)]
pub struct OpenAIComposer {
    model: ComposerEnum,
    client: Client<OpenAIConfig>,
    max_tokens: u16,
}

impl Default for OpenAIComposer {
    fn default() -> Self {
        let org_id = std::env::var("OPENAI_ORG_ID").unwrap();
        let client = Client::with_config(OpenAIConfig::new().with_org_id(org_id));
        let max_tokens = 1000;
        Self {
            model: ComposerEnum::OpenAIGPT35Turbo,
            client,
            max_tokens,
        }
    }
}

impl OpenAIComposer {
    pub fn set_model(&mut self, model: ComposerEnum) {
        self.model = model
    }

    pub fn set_max_tokens(&mut self, max_tokens: u16) {
        self.max_tokens = max_tokens;
    }
}

impl OpenAIComposer {
    pub async fn compose(
        &self,
        context: &str,
        query: &str,
    ) -> Result<(String, Message), Box<dyn Error>> {
        match self.model {
            ComposerEnum::OpenAIGPT35Turbo => {
                self.process_chat_competion_request(context, query).await
            }
            _ => self.process_competion_request(context, query).await,
        }
    }

    async fn process_chat_competion_request(
        &self,
        context: &str,
        query: &str,
    ) -> Result<(String, Message), Box<dyn Error>> {
        let messages = prompt::build_chat_messages(context, query)?;
        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model.to_string())
            .max_tokens(self.max_tokens)
            .messages(messages.to_owned())
            .temperature(0.0)
            .build()?;
        let request_text = format!("{:?}", &messages);
        let response = self.client.chat().create(request).await?;
        match response.choices.first() {
            Some(choice) => Ok((
                request_text,
                choice.message.content.clone().unwrap_or("".to_string()),
            )),
            None => Ok((request_text, "".to_string())),
        }
    }

    async fn process_competion_request(
        &self,
        context: &str,
        query: &str,
    ) -> Result<(String, Message), Box<dyn Error>> {
        let prompt = prompt::build_prompt_content(context, query);
        let request = CreateCompletionRequestArgs::default()
            .model(&self.model.to_string())
            .max_tokens(self.max_tokens)
            .prompt(&prompt)
            .temperature(0.0)
            .build()?;
        let response = self.client.completions().create(request).await?;

        match response.choices.first() {
            Some(choice) => Ok((prompt, choice.text.to_string())),
            None => Ok((prompt, "".to_string())),
        }
    }
}
