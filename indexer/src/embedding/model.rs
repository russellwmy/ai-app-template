use std::str::FromStr;

use super::{MiniLMEmbeddingModel, ModelId};
use crate::Result;

#[derive(Debug)]
pub enum EmbeddingModel {
    MiniLMEmbeddingModel(MiniLMEmbeddingModel),
    // OpenAIAdaV2EmbeddingModel(OpenAIAdaV2EmbeddingModel),
}

impl EmbeddingModel {
    pub fn run(&self, data: &str) -> Result<Vec<f32>> {
        let result = match self {
            EmbeddingModel::MiniLMEmbeddingModel(model) => model.run(data)?,
            // EmbeddingModel::OpenAIAdaV2EmbeddingModel(model) => model.run(data.to_owned()).await?,
        };

        Ok(result)
    }

    pub fn get_model_id(&self) -> ModelId {
        let result = match self {
            EmbeddingModel::MiniLMEmbeddingModel(model) => model.get_model_id(),
            // EmbeddingModel::OpenAIAdaV2EmbeddingModel(model) => model.get_model_id(),
        };

        result
    }

    pub fn load_model(model_id: &str, model_file: Option<&str>) -> Result<EmbeddingModel> {
        let model_id = ModelId::from_str(&model_id)?;

        let result = match model_id {
            ModelId::AllMiniLML12V2 => {
                let model =
                    MiniLMEmbeddingModel::from_file(ModelId::AllMiniLML12V2, &model_file.unwrap())?;
                EmbeddingModel::MiniLMEmbeddingModel(model)
            }
            ModelId::AllMiniLML6V2 => {
                let model =
                    MiniLMEmbeddingModel::from_file(ModelId::AllMiniLML6V2, &model_file.unwrap())?;
                EmbeddingModel::MiniLMEmbeddingModel(model)
            } // ModelId::OpenAITextEmbeddingAdaV2 => {
              //     let model = OpenAIAdaV2EmbeddingModel::new(ModelId::OpenAITextEmbeddingAdaV2)?;
              //     EmbeddingModel::OpenAIAdaV2EmbeddingModel(model)
              // }
        };

        Ok(result)
    }
}

// impl From<OpenAIAdaV2EmbeddingModel> for EmbeddingModel {
//     fn from(model: OpenAIAdaV2EmbeddingModel) -> Self {
//         EmbeddingModel::OpenAIAdaV2EmbeddingModel(model)
//     }
// }

impl From<MiniLMEmbeddingModel> for EmbeddingModel {
    fn from(model: MiniLMEmbeddingModel) -> Self {
        EmbeddingModel::MiniLMEmbeddingModel(model)
    }
}
