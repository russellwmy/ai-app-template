use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

pub type NodeId = String;
pub type RankId = String;

#[derive(Serialize, Deserialize, Debug, Clone, TypedBuilder, Getters)]
pub struct Node {
    #[builder(default = None)]
    pub parent_id: Option<String>,
    pub id: NodeId,
    pub data: String,
    pub rank_id: RankId,
    pub hash: String,
    pub embeddings: Vec<f32>,
    #[builder(default = None)]
    pub reference: Option<String>, // reference document
}

impl Default for Node {
    fn default() -> Self {
        Self {
            parent_id: Default::default(),
            id: Default::default(),
            data: Default::default(),
            rank_id: Default::default(),
            hash: Default::default(),
            embeddings: Default::default(),
            reference: Default::default(),
        }
    }
}
