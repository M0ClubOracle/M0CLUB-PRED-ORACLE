
use anchor_lang::prelude::*;

#[event]
pub struct RegistryInitialized {
    pub authority: Pubkey,
    pub created_at_slot: u64,
}

#[event]
pub struct MarketUpserted {
    pub market_id: String,
    pub active: bool,
}

#[event]
pub struct AuthoritySet {
    pub new_authority: Pubkey,
}
