use std::str::FromStr;

use crate::Environment;

pub fn get_slack_api_token() -> Result<String, std::env::VarError> {
    std::env::var("SLACK_BOT_TOKEN")
}

pub fn get_slack_client_id() -> Result<String, std::env::VarError> {
    std::env::var("SLACK_CLIENT_ID")
}

pub fn get_slack_client_secret() -> Result<String, std::env::VarError> {
    std::env::var("SLACK_CLIENT_SECRET")
}

pub fn get_aws_cognito_client_id() -> Result<String, std::env::VarError> {
    std::env::var("AWS_COGNITO_CLIENT_ID")
}

pub fn get_aws_cognito_user_pool_id() -> Result<String, std::env::VarError> {
    std::env::var("AWS_COGNITO_USER_POOL_ID")
}

pub fn get_app_environment() -> Result<Environment, std::env::VarError> {
    let s = std::env::var("APP_ENVIRONMENT")?;
    Environment::from_str(&s).map_err(|_| std::env::VarError::NotPresent)
}

pub fn get_app_resources_path() -> Result<String, std::env::VarError> {
    Ok("/mnt/resources".to_string())
}

pub fn get_app_index_bucket() -> Result<String, std::env::VarError> {
    let value = match get_app_environment()? {
        Environment::Production => "app-index".to_string(),
        _ => "app-index-stage".to_string(),
    };
    Ok(value)
}

pub fn get_app_tasks_bucket() -> Result<String, std::env::VarError> {
    let value = match get_app_environment()? {
        Environment::Production => "app-tasks".to_string(),
        _ => "app-tasks-stage".to_string(),
    };
    Ok(value)
}

pub fn get_app_uploads_bucket() -> Result<String, std::env::VarError> {
    let value = match get_app_environment()? {
        Environment::Production => "app-uploads".to_string(),
        _ => "app-uploads-stage".to_string(),
    };
    Ok(value)
}

pub fn get_app_document_bucket() -> Result<String, std::env::VarError> {
    let value = match get_app_environment()? {
        Environment::Production => "app-document".to_string(),
        _ => "app-document-stage".to_string(),
    };
    Ok(value)
}

pub fn get_app_mq_bucket() -> Result<String, std::env::VarError> {
    let value = match get_app_environment()? {
        Environment::Production => "app-mq".to_string(),
        _ => "app-mq-stage".to_string(),
    };
    Ok(value)
}
