use std::{fmt::format, path::PathBuf};

use aws_sdk_s3::{primitives::ByteStream, Client};
use document::{chunking::OverlappedChunker, docx::DocxDcoumentPraser, pdf::PdfDocumentParser};
use rayon::prelude::*;
use tracing::info;

use crate::{
    graph::{Graph, Node},
    math,
    Context,
    EmbeddingModel,
    Indexer,
    IndexingMeta,
    MiniLMEmbeddingModel,
    ModelId,
};

const GRAPH_FILENAME: &str = "embedding.json";
const DATA_FILENAME: &str = "data.json";
const PDFIUM_LIB_PATH: &str = "lib/libpdfium.so";

use crate::Result;

pub async fn build_index(
    client: &Client,
    meta: &IndexingMeta,
    document_key: &str,
    file_key: &str,
    keep_file: bool,
) -> Result<()> {
    let bucket_name = common::vars::get_app_document_bucket()?;
    let resources_path = common::vars::get_app_resources_path()?;
    let filename = file_key.split("/").last().unwrap();

    // Download task
    let output = s3_helper::download_object(&client, &bucket_name, &file_key).await?;
    let data = output.body.collect().await.map(|data| data.into_bytes())?;

    let document = if filename.ends_with(".docx") {
        let parser = DocxDcoumentPraser::new();
        let document = parser.parse(data.to_vec())?;

        document
    } else if filename.ends_with(".pdf") {
        let mut pdfium_lib_path = PathBuf::from(&resources_path);
        pdfium_lib_path.push(PDFIUM_LIB_PATH);
        let parser = PdfDocumentParser::new(pdfium_lib_path.to_str().unwrap())?;
        let document = parser.parse(data.to_vec())?;
        document
    } else {
        panic!("Unsupported file format.")
    };
    info!("start index: {}", filename);
    let content = serde_json::to_string(&document)?;
    let mut output_key = PathBuf::from(document_key);
    output_key.push(DATA_FILENAME);
    s3_helper::upload_object_with_content(
        &client,
        &bucket_name,
        output_key.to_str().unwrap(),
        ByteStream::from(content.as_bytes().to_vec()),
    )
    .await?;
    info!("uploaded txt file");

    let chunker = OverlappedChunker::with_size(500);
    let chunks = chunker.chunks(&document.all_text_in_lines());
    let texts = chunks.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    info!("splitted content");

    let model_id = ModelId::AllMiniLML6V2.to_string();
    let model_name = model_id.split("::").last().unwrap_or("");
    let mut path = PathBuf::from(&resources_path);
    path.push("models");
    path.push(model_name);
    let model_path = path.to_str().unwrap();
    let embedding_model = MiniLMEmbeddingModel::from_file(ModelId::AllMiniLML6V2, model_path)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let model = EmbeddingModel::MiniLMEmbeddingModel(embedding_model);
    info!("loaded model: {}", model_id);

    let indexer = Indexer::new(model).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let graph = indexer
        .index(texts, meta.clone())
        .await
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    info!("indexed doc: {}", &file_key);
    let output_content = graph.to_string();

    let mut output_key = PathBuf::from(document_key);
    output_key.push(GRAPH_FILENAME);

    s3_helper::upload_object_with_content(
        &client,
        &bucket_name,
        output_key.to_str().unwrap(),
        ByteStream::from(output_content.as_bytes().to_vec()),
    )
    .await?;
    info!("uploaded indexed file: {}", file_key);
    info!("indexed document: {}", document_key);

    if !keep_file {
        s3_helper::delete_object(&client, &bucket_name, &file_key).await?;
    }
    Ok(())
}

pub async fn get_embedding_model(resources_path: &str) -> Result<EmbeddingModel> {
    let model_id = ModelId::AllMiniLML6V2.to_string();
    let model_name = model_id.split("::").last().unwrap_or("");
    let mut path = PathBuf::from(resources_path);
    path.push("models");
    path.push(model_name);
    let model_path = path.to_str().unwrap();
    let embedding_model = MiniLMEmbeddingModel::from_file(ModelId::AllMiniLML6V2, model_path)
        .expect("fail to load model.");
    let model = EmbeddingModel::MiniLMEmbeddingModel(embedding_model);

    Ok(model)
}

pub async fn search_graph(
    graphs: Vec<Graph>,
    query: &str,
    model: &EmbeddingModel,
) -> Result<Vec<(f32, String)>> {
    info!("search nodes");
    let query_embedding = model
        .run(query)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let chunker = OverlappedChunker::with_size(100);

    let mut results: Vec<(f32, String)> = vec![];
    for graph in graphs {
        let query_embedding: Vec<f32> = query_embedding.clone();
        let nodes = graph.get_all_nodes();
        let title = graph.title();

        let mut document_chunks = nodes
            .par_iter()
            .map(|node| {
                let similarity =
                    math::cosine_similarity(query_embedding.to_vec(), node.embeddings().to_vec());
                (similarity, node.to_owned())
            })
            .collect::<Vec<(f32, Node)>>();
        document_chunks.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        let new_data: Vec<String> = document_chunks
            .par_iter()
            .take(10)
            .map(|(_, node)| {
                node.data()
                    .split("\n")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect::<Vec<String>>();

        let chunks = chunker.chunks(&new_data);
        let result = chunks
            .par_iter()
            .map(|chunk| {
                let embedding_result = model
                    .run(&chunk)
                    .map_err(|e| anyhow::anyhow!(e.to_string()))
                    .unwrap();
                let similarity =
                    math::cosine_similarity(query_embedding.to_vec(), embedding_result.to_vec());
                let text = format!("From document {}:\n {}", title, chunk.to_owned());
                (similarity, text)
            })
            .collect::<Vec<(f32, String)>>();
        results.extend(result);
    }

    results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    info!("search result: {}", results.len());
    Ok(results)
}

pub async fn load_graphs_from_s3(
    client: &Client,
    bucket: &str,
    document_keys: Vec<&str>,
) -> Result<Vec<Graph>> {
    let mut graphs = vec![];
    for document_key in document_keys {
        let graph_file_key = format!("{}/{}", document_key, GRAPH_FILENAME);
        info!("load graph: {:?}", graph_file_key);
        match s3_helper::download_object(&client, &bucket, &graph_file_key).await {
            Ok(output) => {
                let data = output.body.collect().await.map(|data| data.into_bytes())?;
                let obj = serde_json::from_slice(&data)?;
                let graph = Graph::from(obj);
                graphs.push(graph);
            }
            Err(_) => panic!("Fail to read {}", graph_file_key),
        };
    }
    Ok(graphs)
}

pub async fn search_context(
    graphs: Vec<Graph>,
    query: &str,
    model: &EmbeddingModel,
    max_tokens: usize,
) -> Result<Vec<Context>> {
    info!("search context");
    let query_embedding = model
        .run(query)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let chunker = OverlappedChunker::with_size(100);

    let mut results: Vec<Context> = vec![];
    for graph in graphs {
        let query_embedding: Vec<f32> = query_embedding.clone();
        let nodes = graph.get_all_nodes();
        let title = graph.title();
        let reference = graph.reference();

        info!("search graph: {}", title);

        let mut document_chunks = nodes
            .par_iter()
            .map(|node| {
                let similarity =
                    math::cosine_similarity(query_embedding.to_vec(), node.embeddings().to_vec());
                (similarity, node.to_owned())
            })
            .collect::<Vec<(f32, Node)>>();
        document_chunks.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        let new_data: Vec<String> = document_chunks
            .par_iter()
            .take(10)
            .map(|(_, node)| {
                node.data()
                    .split("\n")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect::<Vec<String>>();

        let chunks = chunker.chunks(&new_data);
        let result = chunks
            .par_iter()
            .map(|chunk| {
                let embedding_result = model
                    .run(&chunk)
                    .map_err(|e| anyhow::anyhow!(e.to_string()))
                    .unwrap();
                let score =
                    math::cosine_similarity(query_embedding.to_vec(), embedding_result.to_vec());
                let raw_data = chunk.to_owned();
                let data = format!("From document {}:\n {}", title, raw_data);
                Context::builder()
                    .score(score)
                    .raw_data(raw_data)
                    .data(data)
                    .reference(reference.to_owned())
                    .build()
            })
            .collect::<Vec<Context>>();
        results.extend(result);
    }

    results.sort_by(|a, b| b.score().partial_cmp(&a.score()).unwrap());
    let mut contexts = vec![];
    let mut current_tokens = 0;

    for context in results {
        current_tokens = current_tokens + context.data().len() / 4;

        if current_tokens > max_tokens {
            break;
        }
        contexts.push(context);
    }
    Ok(contexts)
}
