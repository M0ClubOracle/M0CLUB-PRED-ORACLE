
use anchor_lang::prelude::*;

#[account]
pub struct CommitRecord {
    pub market: Pubkey,
    pub epoch: Pubkey,
    pub committer: Pubkey,
    pub commit_hash: [u8; 32],
    pub reveal_after_slot: u64,
    pub revealed: bool,
    pub bump: u8,
}

impl CommitRecord {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 32 + 8 + 1 + 1;
}
