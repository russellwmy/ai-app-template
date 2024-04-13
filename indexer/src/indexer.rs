use std::collections::HashMap;

use anyhow::Result;
use common::generate_id;
use rayon::prelude::*;

use crate::{
    embedding::EmbeddingModel,
    graph::{Graph, Node},
    IndexingMeta,
};

#[derive(Debug)]
pub struct Indexer {
    model: EmbeddingModel,
}

impl Indexer {
    pub fn new(model: EmbeddingModel) -> Result<Self> {
        Ok(Self { model })
    }

    pub async fn index(&self, texts: Vec<&str>, meta: IndexingMeta) -> Result<Graph> {
        let reference = meta.id();
        let mut node_map = HashMap::new();

        let nodes = texts
            .par_iter()
            .enumerate()
            .map(|(index, text)| {
                let result = self.model.run(text).unwrap();
                let node = Node::builder()
                    .id(common::generate_id_with_data(text))
                    .hash(common::hash(text.as_bytes()))
                    .rank_id(format!("{}::{}", reference.to_string(), index))
                    .reference(Some(reference.to_string()))
                    .data(text.to_string())
                    .embeddings(result)
                    .build();
                node
            })
            .collect::<Vec<Node>>();

        for node in nodes {
            node_map.insert(node.id().to_string(), node);
        }

        let graph = Graph::builder()
            .id(generate_id())
            .title(meta.title().to_string())
            .node_map(node_map)
            .index_model(Some(self.model.get_model_id()))
            .reference(Some(reference.to_string()))
            .reference_link(Some(meta.external_link().to_string()))
            .build();

        Ok(graph)
    }
}
