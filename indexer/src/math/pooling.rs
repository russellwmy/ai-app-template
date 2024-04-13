use std::{iter, ops::Mul};

use ndarray::{ArrayBase, Axis, Dim, IxDynImpl, OwnedRepr};
use tokenizers::tokenizer::Result;
use tract_onnx::{prelude::*, tract_hir::internal::tract_smallvec::SmallVec};

// Mean Pooling - Take attention mask into account for correct averaging
pub fn mean_pooling(
    model_output: SmallVec<[TValue; 4]>,
    attention_mask: &[u32],
) -> Result<ArrayBase<OwnedRepr<f32>, Dim<IxDynImpl>>> {
    let token_embeddings = model_output[0].clone().into_tensor().into_array::<f32>()?;
    let shape = token_embeddings.shape();
    let new_attention_mask = tract_ndarray::Array2::from_shape_vec(
        (1, token_embeddings.len()),
        attention_mask
            .iter()
            .map(|&x| iter::repeat(x as f32).take(shape[2]))
            .flatten()
            .collect(),
    )?;
    let new_attention_mask = new_attention_mask.into_shape::<&[usize]>(shape.into())?;

    let sum_result = token_embeddings.mul(&new_attention_mask).sum_axis(Axis(1));
    let attention_mask_result = new_attention_mask
        .sum_axis(Axis(1))
        .map(|x| x.clamp(1e-9, f32::MAX));
    let result = sum_result / attention_mask_result;

    Ok(result)
}
