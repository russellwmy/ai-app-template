use anyhow::Result;
use regex::Regex;
use reqwest::Url;
use tracing::info;

pub fn is_google_docs_url(url: &str) -> bool {
    let parsed_url = Url::parse(url).unwrap();
    parsed_url.host_str().unwrap().contains("docs.google.com")
}

pub fn parse_google_docs_url(url: &str) -> Result<String> {
    let re = Regex::new(r"[-\w]{25,}")?;
    let caps = re.captures(&url).unwrap();
    let file_id = caps.get(0).map_or("", |m| m.as_str());

    let url = format!(
        "https://docs.google.com/document/u/0/export?format=docx&id={}",
        file_id
    );
    Ok(url)
}
