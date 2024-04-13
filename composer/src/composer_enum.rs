use core::fmt;
use std::{error::Error, str::FromStr};

#[derive(Debug)]
pub enum ComposerEnum {
    OpenAIAdaV1,
    OpenAIBabbageV1,
    OpenAICurieV1,
    OpenAIDavinciV3,
    OpenAIGPT35Turbo,
}

impl FromStr for ComposerEnum {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "text-ada-001" => ComposerEnum::OpenAIAdaV1,
            "text-babbage-001" => ComposerEnum::OpenAIBabbageV1,
            "text-curie-001" => ComposerEnum::OpenAICurieV1,
            "text-davinci-003" => ComposerEnum::OpenAIDavinciV3,
            "gpt-3.5-turbo" => ComposerEnum::OpenAIGPT35Turbo,
            _ => todo!(),
        };

        Ok(result)
    }
}

impl fmt::Display for ComposerEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ComposerEnum::OpenAIAdaV1 => write!(f, "text-ada-001"),
            ComposerEnum::OpenAIBabbageV1 => write!(f, "text-babbage-001"),
            ComposerEnum::OpenAICurieV1 => write!(f, "text-curie-001"),
            ComposerEnum::OpenAIDavinciV3 => write!(f, "text-davinci-003"),
            ComposerEnum::OpenAIGPT35Turbo => write!(f, "gpt-3.5-turbo"),
        }
    }
}
