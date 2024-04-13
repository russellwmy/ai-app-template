use std::{collections::HashMap, error::Error, str::FromStr};

use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{node::NodeId, Node};
use crate::embedding::ModelId;

pub type GraphId = String;

#[derive(Serialize, Deserialize, Debug, TypedBuilder, Getters)]
pub struct Graph {
    id: GraphId,
    title: String,
    node_map: HashMap<NodeId, Node>,
    index_model: Option<ModelId>,
    #[builder(default = None)]
    embeddings: Option<Vec<f32>>,
    #[builder(default = None)]
    reference: Option<String>,
    #[builder(default = None)] // reference document
    reference_link: Option<String>, // reference document
    #[builder(default = None)]
    hash: Option<String>,
}

impl Graph {
    pub fn node_count(&self) -> usize {
        self.node_map.values().count()
    }

    pub fn get_all_nodes(&self) -> Vec<Node> {
        self.node_map.clone().into_values().collect()
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            id: common::generate_id(),
            node_map: HashMap::new(),
            title: "No name".to_string(),
            hash: None,
            index_model: None,
            embeddings: None,
            reference: None,
            reference_link: None,
        }
    }
}

impl FromStr for Graph {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: Graph = serde_json::from_str(&s)?;

        Ok(result)
    }
}

impl ToString for Graph {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
