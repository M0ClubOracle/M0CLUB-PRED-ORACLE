
use anchor_lang::prelude::*;

#[account]
pub struct Epoch {
    pub market: Pubkey,
    pub epoch_id: u64,
    pub open: bool,
    pub opened_at_slot: u64,
    pub finalized_at_slot: u64,
    pub publish_sequence: u64, // replay protection for reveals
    pub bump: u8,
}

impl Epoch {
    pub const LEN: usize = 8 + 32 + 8 + 1 + 8 + 8 + 8 + 1;
}
