
use anchor_lang::prelude::*;
use crate::events::*;
use crate::state::metadata::{MarketMetadata, MARKET_META_SEED};
use crate::state::registry::{Registry, REGISTRY_SEED};
use crate::error::M0RegistryError;

const MAX_OUTCOME_LEN: usize = 64;

#[derive(Accounts)]
#[instruction(market_id: String)]
pub struct UpsertMarket<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [REGISTRY_SEED],
        bump = registry.bump,
        has_one = authority @ M0RegistryError::Unauthorized
    )]
    pub registry: Account<'info, Registry>,

    #[account(
        init_if_needed,
        payer = authority,
        space = MarketMetadata::len_with(market_id.len().max(1), outcomes.len().max(1), MAX_OUTCOME_LEN),
        seeds = [MARKET_META_SEED, market_id.as_bytes()],
        bump
    )]
    pub meta: Account<'info, MarketMetadata>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<UpsertMarket>,
    market_id: String,
    domain: String,
    cadence_ms: u32,
    tier_policy: String,
    outcomes: Vec<String>,
    active: bool,
) -> Result<()> {
    if market_id.is_empty() || market_id.len() > 64 {
        return err!(M0RegistryError::InvalidParameter);
    }
    if outcomes.is_empty() || outcomes.len() > 16 {
        return err!(M0RegistryError::InvalidParameter);
    }

    let m = &mut ctx.accounts.meta;
    let now = Clock::get()?.slot;

    let is_new = m.created_at_slot == 0;
    if is_new {
        ctx.accounts.registry.market_count = ctx.accounts.registry.market_count.saturating_add(1);
        m.created_at_slot = now;
    }

    m.market_id = market_id.clone();
    m.domain = domain;
    m.cadence_ms = cadence_ms;
    m.tier_policy = tier_policy;
    m.outcomes = outcomes;
    m.active = active;
    m.updated_at_slot = now;
    m.bump = *ctx.bumps.get("meta").unwrap();

    emit!(MarketUpserted { market_id, active });

    Ok(())
}
