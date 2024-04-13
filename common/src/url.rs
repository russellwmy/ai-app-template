use url::Url;

use crate::Result;

pub fn validate_url(url: &str) -> Result<()> {
    Url::parse(url)?;
    Ok(())
}

pub fn parse_url(url: &str) -> Result<Url> {
    Ok(Url::parse(url)?)
}

pub fn clean_url(url: &str) -> String {
    match url.starts_with("//") {
        true => format!("https:{}", url),
        false => url.to_string(),
    }
}
