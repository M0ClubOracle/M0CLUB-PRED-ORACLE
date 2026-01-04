
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct OutcomePoint {
    pub outcome_id: String,
    pub p_scaled: u64,
    pub ci_low_scaled: u64,
    pub ci_high_scaled: u64,
    pub ci_level_bps: u16,
    pub quality_flags: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MarketReveal {
    pub market_id: String,
    pub epoch_id: u64,
    pub tick_index: u32,
    pub sequence: u64,
    pub observed_at_ms: u64,
    pub risk_score: u16,
    pub quality_flags: u32,
    pub outcomes: Vec<OutcomePoint>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BundleReveal {
    pub schema_version: u16,
    pub signer_set_id: u64,
    pub publish_epoch_id: u64,
    pub created_at_ms: u64,
    pub bundle_id: [u8; 16],
    pub markets: Vec<MarketReveal>,
}
