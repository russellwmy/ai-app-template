use crate::{Composer, ComposerEnum, OpenAIComposer, Result};
use indexer::Context;
use tracing::info;

pub async fn get_composer(max_output_tokens: usize) -> Result<Composer> {
    let mut composer = OpenAIComposer::default();
    composer.set_model(ComposerEnum::OpenAIGPT35Turbo);
    composer.set_max_tokens(max_output_tokens.try_into().unwrap());

    Ok(Composer::OpenAIComposer(composer))
}

pub async fn compose_message_with_graph(
    composer: &Composer,
    nodes: Vec<(f32, String)>,
    query: &str,
    max_input_tokens: usize,
) -> Result<(String, String)> {
    info!("compose message");
    let max_num_token = max_input_tokens;
    let mut contexts = vec![];
    let mut current_tokens = 0;

    for (idx, node) in nodes.iter().enumerate() {
        current_tokens += node.1.chars().count() / 4;
        if current_tokens > max_num_token {
            println!("{} contents found", nodes.len());
            println!("{} contents used", idx);
            break;
        }
        let context = node.1.as_str();
        contexts.push(context);
        println!(
            "Chunk: {} Score: {:2} => Content: {:?}",
            idx + 1,
            node.0,
            node.1,
        );
    }

    let context = contexts.join("\n");
    let has_context = contexts.len() != 0;
    let message = match has_context {
        true => {
            info!("context: {:?}", context);
            let (prompt, content) = composer
                .compose(&context, &query)
                .await
                .map_err(|e| anyhow::format_err!("fail to compose message. {}", e.to_string()))?;
            info!(
                event = "message_composed",
                prompt = format!("{:?}", prompt),
                response = format!("{:?}", content),
            );
            content
        }
        false => {
            "\nApology, I can't find any relevant information from the given documents for you."
                .to_string()
        }
    };

    Ok((message, context))
}

pub async fn compose_message_with_context(
    composer: &Composer,
    context: String,
    query: &str,
) -> Result<String> {
    info!("compose message");
    let message = match !context.is_empty() {
        true => {
            info!("context: {:?}", context);
            let (prompt, content) = composer
                .compose(&context, &query)
                .await
                .map_err(|e| anyhow::format_err!("fail to compose message. {}", e.to_string()))?;
            info!(
                event = "message_composed",
                prompt = format!("{:?}", prompt),
                response = format!("{:?}", content),
            );
            content
        }
        false => {
            "\nApology, I can't find any relevant information from the given documents for you."
                .to_string()
        }
    };

    Ok(message)
}
