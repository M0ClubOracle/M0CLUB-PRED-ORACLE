
use anchor_lang::prelude::*;

pub const ROUTER_SEED: &[u8] = b"router";

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Route {
    pub destination: Pubkey,
    pub bps: u16, // 0..10000
}

#[account]
pub struct Router {
    pub authority: Pubkey,
    pub routes: Vec<Route>,
    pub bump: u8,
}

impl Router {
    pub fn len_with(routes_len: usize) -> usize {
        // routes: 4 + routes_len * (32 + 2)
        8 + 32 + 4 + routes_len * (32 + 2) + 1
    }
}
