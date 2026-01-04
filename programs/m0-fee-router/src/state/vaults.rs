
use anchor_lang::prelude::*;

pub const VAULT_SEED: &[u8] = b"vault";

#[account]
pub struct FeeVault {
    pub router: Pubkey,
    pub bump: u8,
}

impl FeeVault {
    pub const LEN: usize = 8 + 32 + 1;
}
