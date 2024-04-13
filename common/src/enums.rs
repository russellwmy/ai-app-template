use std::str::FromStr;

use base64ct::Error;

pub enum Environment {
    Development,
    Stage,
    Production,
}

impl FromStr for Environment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "production" => Self::Production,
            "stage" => Self::Stage,
            _ => Self::Development,
        };
        Ok(result)
    }
}
