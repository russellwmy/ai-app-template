use std::error::Error;

use async_openai::{config::OpenAIConfig, types::CreateEmbeddingRequestArgs, Client};

use super::ModelId;

#[derive(Debug)]
pub struct OpenAIAdaV2EmbeddingModel {
    client: Client<OpenAIConfig>,
    model: String,
    model_id: ModelId,
}

impl OpenAIAdaV2EmbeddingModel {
    pub fn new(model_id: ModelId) -> Result<Self, Box<dyn Error>> {
        let model = model_id.to_string();
        let model = model.split("::").last().unwrap();
        let org_id = std::env::var("OPENAI_ORG_ID")?;
        let client = Client::with_config(OpenAIConfig::new().with_org_id(org_id));

        Ok(Self {
            client,
            model_id,
            model: model.to_string(),
        })
    }

    pub fn get_model_id(&self) -> ModelId {
        self.model_id.clone()
    }

    pub async fn run(&self, data: Vec<&str>) -> Result<Vec<Vec<f32>>, Box<dyn Error>> {
        let text = data.join("\n");
        let tokens = text.split_whitespace();
        println!("Token used: {}", tokens.count());

        let client = self.client.to_owned();
        let request = CreateEmbeddingRequestArgs::default()
            .model(self.model.to_owned())
            .input(data)
            .build()?;

        let mut embeddings = vec![];
        let response = client.embeddings().create(request).await?;
        for embedding in response.data {
            embeddings.push(embedding.embedding.to_vec())
        }
        Ok(embeddings)
    }
}
