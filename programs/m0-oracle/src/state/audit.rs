
use anchor_lang::prelude::*;

#[account]
pub struct AuditLog {
    pub market: Pubkey,
    pub epoch: Pubkey,
    pub last_bundle_hash: [u8; 32],
    pub last_sequence: u64,
    pub last_revealed_at_slot: u64,
    pub bump: u8,
}

impl AuditLog {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 8 + 8 + 1;
}
