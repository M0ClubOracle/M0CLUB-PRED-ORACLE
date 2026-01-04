
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FeatureStoreError {
    #[error("storage error: {0}")]
    Storage(String),
    #[error("transform error: {0}")]
    Transform(String),
}
