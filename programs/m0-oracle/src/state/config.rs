
use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
pub struct ProtocolConfig {
    pub initialized: bool,
    pub authority: Pubkey,
    pub paused: bool,
    pub next_market_nonce: u64,
    pub next_signer_set_id: u64,
    pub default_reveal_delay_slots: u64,
    pub bump: u8,
}

impl ProtocolConfig {
    pub const LEN: usize = 8 + 1 + 32 + 1 + 8 + 8 + 8 + 1;
}

pub fn protocol_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[PROTOCOL_SEED], &crate::ID)
}
