
use crate::error::SignerError;

pub async fn submit_tx_simulated(_cluster: &str, _payload: &[u8]) -> Result<String, SignerError> {
    // Placeholder: return synthetic tx signature.
    Ok("SIMULATED_TX_SIG".to_string())
}
