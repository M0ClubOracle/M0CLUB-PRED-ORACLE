
use crate::error::SignerError;

#[derive(Debug, Default, Clone)]
pub struct ReplayState {
    pub last_sequence: u64,
}

impl ReplayState {
    pub fn next(&mut self) -> Result<u64, SignerError> {
        self.last_sequence = self.last_sequence.saturating_add(1);
        Ok(self.last_sequence)
    }
}
