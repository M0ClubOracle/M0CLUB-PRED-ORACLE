
use anchor_lang::prelude::*;

pub const REGISTRY_SEED: &[u8] = b"registry";

#[account]
pub struct Registry {
    pub initialized: bool,
    pub authority: Pubkey,
    pub market_count: u64,
    pub bump: u8,
}

impl Registry {
    pub const LEN: usize = 8 + 1 + 32 + 8 + 1;
}
