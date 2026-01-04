
use crate::error::SignerError;

#[derive(Debug, Clone)]
pub struct KmsKey {
    pub key_id: String,
}

impl KmsKey {
    pub fn new(key_id: impl Into<String>) -> Self {
        Self { key_id: key_id.into() }
    }

    pub fn sign(&self, _msg: &[u8]) -> Result<Vec<u8>, SignerError> {
        // Placeholder. Integrate AWS KMS / GCP KMS / HashiCorp Vault later.
        Ok(vec![])
    }
}
