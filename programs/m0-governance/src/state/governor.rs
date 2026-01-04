
use anchor_lang::prelude::*;

pub const GOVERNOR_SEED: &[u8] = b"governor";
pub const PROPOSAL_SEED: &[u8] = b"proposal";

#[account]
pub struct Governor {
    pub authority: Pubkey,
    pub guardians: Vec<Pubkey>,
    pub voting_period_slots: u64,
    pub quorum_bps: u16,
    pub proposal_count: u64,
    pub bump: u8,
}

impl Governor {
    pub fn len_with(guardians_len: usize) -> usize {
        8 + 32 + 4 + 32*guardians_len + 8 + 2 + 8 + 1
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Action {
    pub program_id: Pubkey,
    pub accounts: Vec<Pubkey>,
    pub data: Vec<u8>,
}

#[account]
pub struct Proposal {
    pub governor: Pubkey,
    pub proposer: Pubkey,
    pub proposal_id: u64,
    pub created_at_slot: u64,
    pub voting_ends_at_slot: u64,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub executed: bool,
    pub actions: Vec<Action>,
    pub bump: u8,
}

impl Proposal {
    pub fn len_with(actions_len: usize, max_data: usize, max_accounts: usize) -> usize {
        // Very conservative sizing; tune per your needs.
        8 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 1 + 4 + actions_len * (32 + 4 + max_accounts*32 + 4 + max_data) + 1
    }
}
