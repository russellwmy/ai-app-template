use async_openai::{
    error::OpenAIError,
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs, Role,
    },
};

pub fn build_prompt_content(context: &str, query: &str) -> String {
    format!(
        "You are an expert of the knowledge given to you. \
            Answer the question based on the context below. \
            If the question cannot be answered using the knowledge provided below ######, \
            answer with a humorous disclaimer.\
            \n\n\
            {}
            \n\n\
            {}",
        context, query
    )
}

pub fn build_chat_messages(
    context: &str,
    query: &str,
) -> Result<Vec<ChatCompletionRequestMessage>, OpenAIError> {
    let system_message = format!(
        "You are an expert of the knowledge given to you. \
        Answer the question based on the context below. \
        If the question cannot be answered using the knowledge provided below ######, \
        answer with a humorous disclaimer.\
        \n\n\
        {}",
        context
    );
    let qa_pairs = vec![];
    let messages = chat_messages(&system_message, qa_pairs, query)?;
    Ok(messages)
}

fn chat_messages(
    system_message: &str,
    pairs: Vec<(&str, &str)>,
    query: &str,
) -> Result<Vec<ChatCompletionRequestMessage>, OpenAIError> {
    let mut result = vec![ChatCompletionRequestSystemMessageArgs::default()
        .content(system_message)
        .build()
        .unwrap()
        .into()];
    let qs_result = pairs.iter().fold(
        vec![],
        |mut acc: Vec<ChatCompletionRequestMessage>, (q, a)| {
            acc.push(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(q.to_owned())
                    .build()
                    .unwrap()
                    .into(),
            );
            acc.push(
                ChatCompletionRequestAssistantMessageArgs::default()
                    .content(a.to_owned())
                    .build()
                    .unwrap()
                    .into(),
            );
            acc
        },
    );

    result.extend(qs_result);
    result.push(
        ChatCompletionRequestUserMessageArgs::default()
            .content(query)
            .build()
            .unwrap()
            .into(),
    );

    Ok(result)
}
