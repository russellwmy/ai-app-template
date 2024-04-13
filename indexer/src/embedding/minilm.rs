use std::path::{Path, PathBuf};

use ndarray::{ArrayBase, IxDynImpl, OwnedRepr};
use tokenizers::Tokenizer;
use tract_onnx::prelude::*;

use super::ModelId;
use crate::{math, Result};

#[derive(Debug)]
pub struct MiniLMEmbeddingModel {
    tokenizer: Tokenizer,
    model_id: ModelId,
    model: SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>,
}

impl MiniLMEmbeddingModel {
    pub fn from_file(model_id: ModelId, model_path: &str) -> Result<Self> {
        let model_dir = PathBuf::from(model_path);
        let tokenizer = Tokenizer::from_file(Path::join(&model_dir, "tokenizer.json")).unwrap();
        let model = tract_onnx::onnx()
            .model_for_path(Path::join(&model_dir, "model.onnx"))?
            .into_optimized()?
            .into_runnable()?;

        Ok(Self {
            tokenizer,
            model,
            model_id,
        })
    }

    pub fn get_model_id(&self) -> ModelId {
        self.model_id.clone()
    }

    pub fn run(&self, text: &str) -> Result<Vec<f32>> {
        let result = self.sentence_embeddings(&text)?;
        Ok(result.into_raw_vec())
    }
}
impl MiniLMEmbeddingModel {
    fn sentence_embeddings(
        &self,
        sentence: &str,
    ) -> Result<ArrayBase<OwnedRepr<f32>, ndarray::Dim<IxDynImpl>>> {
        let model = &self.model;
        let tokenizer = &self.tokenizer;
        let encoded_input = tokenizer.encode(sentence, true).unwrap();
        let input_ids = encoded_input.get_ids();
        let attention_mask = encoded_input.get_attention_mask();
        let token_type_ids = encoded_input.get_type_ids();
        let length = encoded_input.len();
        let input_ids: Tensor = tract_ndarray::Array2::from_shape_vec(
            (1, length),
            input_ids.iter().map(|&x| x as i64).collect(),
        )?
        .into();
        let input_attention_mask: Tensor = tract_ndarray::Array2::from_shape_vec(
            (1, length),
            attention_mask.iter().map(|&x| x as i64).collect(),
        )?
        .into();
        let input_token_type_ids: Tensor = tract_ndarray::Array2::from_shape_vec(
            (1, length),
            token_type_ids.iter().map(|&x| x as i64).collect(),
        )?
        .into();

        let outputs = model.run(tvec!(
            input_ids.into(),
            input_attention_mask.into(),
            input_token_type_ids.into()
        ))?;

        let sentence_embeddings = math::mean_pooling(outputs, attention_mask).unwrap();
        let sentence_embeddings = math::normalize(&sentence_embeddings);
        Ok(sentence_embeddings)
    }
}
