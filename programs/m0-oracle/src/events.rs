
use anchor_lang::prelude::*;

#[event]
pub struct ProtocolInitialized {
    pub authority: Pubkey,
    pub created_at_slot: u64,
}

#[event]
pub struct MarketCreated {
    pub market: Pubkey,
    pub market_id: String,
    pub created_at_slot: u64,
}

#[event]
pub struct MarketUpdated {
    pub market: Pubkey,
    pub market_id: String,
    pub active: bool,
    pub updated_at_slot: u64,
}

#[event]
pub struct EpochOpened {
    pub epoch: Pubkey,
    pub market: Pubkey,
    pub epoch_id: u64,
    pub opened_at_slot: u64,
}

#[event]
pub struct PredictionCommitted {
    pub market: Pubkey,
    pub epoch: Pubkey,
    pub committer: Pubkey,
    pub commit_hash: [u8; 32],
    pub reveal_after_slot: u64,
}

#[event]
pub struct PredictionRevealed {
    pub market: Pubkey,
    pub epoch: Pubkey,
    pub revealer: Pubkey,
    pub bundle_hash: [u8; 32],
    pub sequence: u64,
}

#[event]
pub struct EpochFinalized {
    pub epoch: Pubkey,
    pub market: Pubkey,
    pub epoch_id: u64,
    pub finalized_at_slot: u64,
}

#[event]
pub struct SignerSetRotated {
    pub signer_set: Pubkey,
    pub signer_set_id: u64,
    pub threshold: u16,
    pub active: bool,
}

#[event]
pub struct PausedChanged {
    pub paused: bool,
}
