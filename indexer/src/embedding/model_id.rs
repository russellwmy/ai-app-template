use core::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone)]
pub enum ModelId {
    AllMiniLML12V2,
    AllMiniLML6V2,
    // OpenAITextEmbeddingAdaV2,
}
impl FromStr for ModelId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "MiniLM::all-MiniLM-L12-v2" => ModelId::AllMiniLML12V2,
            "MiniLM::all-MiniLM-L6-v2" => ModelId::AllMiniLML6V2,
            // "openai::text-embedding-ada-002" => ModelId::OpenAITextEmbeddingAdaV2,
            _ => todo!(),
        };

        Ok(result)
    }
}
impl fmt::Display for ModelId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModelId::AllMiniLML12V2 => write!(f, "MiniLM::all-MiniLM-L12-v2"),
            ModelId::AllMiniLML6V2 => write!(f, "MiniLM::all-MiniLM-L6-v2"),
            // ModelId::OpenAITextEmbeddingAdaV2 => write!(f, "openai::text-embedding-ada-002"),
        }
    }
}

impl Serialize for ModelId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ModelId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
