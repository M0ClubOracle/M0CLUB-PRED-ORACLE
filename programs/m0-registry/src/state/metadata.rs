
use anchor_lang::prelude::*;

pub const MARKET_META_SEED: &[u8] = b"market_meta";

#[account]
pub struct MarketMetadata {
    pub market_id: String,
    pub domain: String,
    pub cadence_ms: u32,
    pub tier_policy: String,
    pub outcomes: Vec<String>,
    pub active: bool,
    pub created_at_slot: u64,
    pub updated_at_slot: u64,
    pub bump: u8,
}

impl MarketMetadata {
    pub fn len_with(market_id_len: usize, outcomes_len: usize, max_outcome_len: usize) -> usize {
        8 + 4 + market_id_len
        + 4 + 16 // domain small string
        + 4       // cadence_ms
        + 4 + 16  // tier_policy small string
        + 4 + outcomes_len * (4 + max_outcome_len)
        + 1
        + 8 + 8
        + 1
    }
}
