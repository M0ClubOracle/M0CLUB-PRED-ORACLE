
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("pipeline error: {0}")]
    Pipeline(String),
}
