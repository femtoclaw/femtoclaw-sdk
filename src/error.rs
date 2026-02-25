//! SDK Error Types.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SdkError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("API error: {0}")]
    Api(String),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("timeout")]
    Timeout,
}
