
use anchor_lang::prelude::*;

pub const TIMELOCK_SEED: &[u8] = b"timelock";

#[account]
pub struct Timelock {
    pub governor: Pubkey,
    pub min_delay_slots: u64,
    pub bump: u8,
}

impl Timelock {
    pub const LEN: usize = 8 + 32 + 8 + 1;
}
