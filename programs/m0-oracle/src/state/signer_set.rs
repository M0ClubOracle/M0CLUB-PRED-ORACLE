
use anchor_lang::prelude::*;
use crate::error::M0OracleError;

#[account]
pub struct SignerSet {
    pub signer_set_id: u64,
    pub threshold: u16,
    pub pubkeys: Vec<Pubkey>,
    pub active: bool,
    pub created_at_slot: u64,
    pub bump: u8,
}

impl SignerSet {
    pub fn validate(threshold: u16, pubkeys_len: usize) -> Result<()> {
        if threshold == 0 || threshold as usize > pubkeys_len {
            return err!(M0OracleError::InvalidThreshold);
        }
        Ok(())
    }

    pub fn len_with(pubkeys_len: usize) -> usize {
        // pubkeys: 4 + 32*N
        8 + 8 + 2 + 4 + 32 * pubkeys_len + 1 + 8 + 1
    }
}
