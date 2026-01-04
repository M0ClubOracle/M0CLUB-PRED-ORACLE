
use anchor_lang::prelude::*;

pub mod error;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("M0Reg1stry11111111111111111111111111111111");

#[program]
pub mod m0_registry {
    use super::*;

    pub fn init_registry(ctx: Context<init_registry::InitRegistry>) -> Result<()> {
        init_registry::handler(ctx)
    }

    pub fn upsert_market(
        ctx: Context<upsert_market::UpsertMarket>,
        market_id: String,
        domain: String,
        cadence_ms: u32,
        tier_policy: String,
        outcomes: Vec<String>,
        active: bool,
    ) -> Result<()> {
        upsert_market::handler(ctx, market_id, domain, cadence_ms, tier_policy, outcomes, active)
    }

    pub fn set_authority(ctx: Context<set_authority::SetAuthority>, new_authority: Pubkey) -> Result<()> {
        set_authority::handler(ctx, new_authority)
    }
}
