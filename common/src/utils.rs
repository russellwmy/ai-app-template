use crate::{vars::get_app_environment, Environment};

pub fn get_lambda_function_name(name: &str) -> Result<String, std::env::VarError> {
    let value = match get_app_environment()? {
        Environment::Production => name.to_string(),
        _ => format!("{}-stage", name.to_string()),
    };
    Ok(value)
}

pub fn extract_filename_from_content_disposition(content_disposition: &str) -> Option<String> {
    let filename_info = content_disposition
        .split(";")
        .find(|s| s.contains("filename="));
    match filename_info {
        Some(info) => {
            let mut text = info.split("=").last().unwrap().trim().to_string();
            let first_char = text.chars().rev().last().unwrap();
            let last_char = text.chars().last().unwrap();

            if first_char == '\"' && last_char == '\"' {
                let new_text = text[1..text.len() - 2].to_string();
                text = new_text;
            }
            Some(text)
        }
        _ => None,
    }
}

pub fn extract_name_from_filename(filename: &str) -> String {
    let result = std::path::Path::new(filename).file_stem().unwrap();
    result.to_str().unwrap().to_string()
}
