
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NormalizeError {
    #[error("invalid raw event: {0}")]
    Invalid(String),
}
