
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IngestError {
    #[error("connector error: {0}")]
    Connector(String),
    #[error("stream error: {0}")]
    Stream(String),
    #[error("rate limited")]
    RateLimited,
}
