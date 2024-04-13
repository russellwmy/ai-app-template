use std::error::Error as StdError;

use aws_sdk_s3::error::SdkError;

#[derive(thiserror::Error, Debug)]
#[error("unhandled error")]
pub struct Error {
    #[from]
    source: Box<dyn StdError + Send + Sync + 'static>,
}

impl Error {
    pub fn unhandled(source: impl Into<Box<dyn StdError + Send + Sync + 'static>>) -> Self {
        Self {
            source: source.into(),
        }
    }
}

impl From<aws_sdk_s3::error::BuildError> for Error {
    fn from(source: aws_sdk_s3::error::BuildError) -> Self {
        Self::unhandled(source)
    }
}

impl<T> From<SdkError<T>> for Error
where
    T: StdError + Send + Sync + 'static,
{
    fn from(source: SdkError<T>) -> Self {
        Self::unhandled(source)
    }
}
