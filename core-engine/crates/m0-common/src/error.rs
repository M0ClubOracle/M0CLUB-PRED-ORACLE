
use thiserror::Error;

#[derive(Debug, Error)]
pub enum M0Error {
    #[error("configuration error: {0}")]
    Config(String),

    #[error("io error: {0}")]
    Io(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("dependency error: {0}")]
    Dependency(String),

    #[error("internal error: {0}")]
    Internal(String),
}
