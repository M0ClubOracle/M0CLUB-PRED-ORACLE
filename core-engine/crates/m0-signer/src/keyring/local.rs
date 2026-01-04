
use crate::error::SignerError;
use rand::RngCore;

#[derive(Debug, Clone)]
pub struct LocalKey {
    pub secret: [u8; 32],
}

impl LocalKey {
    pub fn generate() -> Self {
        let mut s = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut s);
        Self { secret: s }
    }

    pub fn load_from_env() -> Result<Self, SignerError> {
        // For skeleton: if env var missing, generate ephemeral key.
        Ok(Self::generate())
    }
}
