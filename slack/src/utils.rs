use anyhow::Result;
use slack_morphism::{
    prelude::{SlackHyperClient, SlackOAuthV2AccessTokenRequest, SlackOAuthV2AccessTokenResponse},
    SlackApiToken,
};

pub fn get_slack_api_token() -> Result<SlackApiToken> {
    let token = common::vars::get_slack_api_token()?;
    Ok(SlackApiToken::new(token.into()))
}

pub async fn get_slack_access_token_by_code(
    client: &SlackHyperClient,
    code: &str,
) -> Result<SlackOAuthV2AccessTokenResponse> {
    let client_id = common::vars::get_slack_client_id()?;
    let client_secret = common::vars::get_slack_client_secret()?;
    let request =
        SlackOAuthV2AccessTokenRequest::new(client_id.into(), client_secret.into(), code.into());
    let response = client.oauth2_access(&request).await.unwrap();

    Ok(response)
}
