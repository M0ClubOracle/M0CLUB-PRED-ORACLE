
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignerError {
    #[error("keyring error: {0}")]
    Keyring(String),
    #[error("replay protection error: {0}")]
    Replay(String),
    #[error("tx submission error: {0}")]
    Tx(String),
}
