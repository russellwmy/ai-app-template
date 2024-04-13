use super::{constants, MIMEType};

pub fn all_mime_types() -> Vec<MIMEType> {
    let result = serde_json::from_str(constants::MIME_TYPE_DATA).unwrap_or(vec![]);

    result
}

pub fn get_mime_type_by_extension(ext: &str) -> Option<MIMEType> {
    let mime_types = all_mime_types();
    mime_types.into_iter().find(|o| o.extension == ext)
}

pub fn is_supported_mime_type(mime_type: &str) -> bool {
    constants::SUPPORTED_MIME_TYPES.contains(&mime_type)
}
