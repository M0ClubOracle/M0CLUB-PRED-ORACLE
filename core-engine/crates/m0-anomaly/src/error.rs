
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnomalyError {
    #[error("anomaly detected: {0}")]
    Detected(String),
}
