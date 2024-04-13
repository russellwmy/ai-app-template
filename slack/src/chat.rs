use anyhow::Result;
use slack_morphism::{
    prelude::{SlackApiChatPostEphemeralRequest, SlackApiChatPostMessageRequest, SlackHyperClient},
    SlackApiToken, SlackChannelId, SlackMessageContent, SlackUserId,
};

pub async fn post_ephemeral(
    client: &SlackHyperClient,
    token: SlackApiToken,
    channel_id: SlackChannelId,
    user_id: SlackUserId,
    msg: &str,
) -> Result<()> {
    let session = client.open_session(&token);
    let content = SlackMessageContent::new().with_text(msg.to_string());
    let req = SlackApiChatPostEphemeralRequest::new(channel_id, user_id, content.clone());
    session.chat_post_ephemeral(&req).await?;

    Ok(())
}

pub async fn post(
    client: &SlackHyperClient,
    token: SlackApiToken,
    channel_id: SlackChannelId,
    msg: &str,
) -> Result<()> {
    let session = client.open_session(&token);
    let content = SlackMessageContent::new().with_text(msg.to_string());
    let req = SlackApiChatPostMessageRequest::new(channel_id, content.clone());
    session.chat_post_message(&req).await?;

    Ok(())
}
