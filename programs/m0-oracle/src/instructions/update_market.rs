
use anchor_lang::prelude::*;
use crate::constants::*;
use crate::events::*;
use crate::error::M0OracleError;
use crate::state::config::ProtocolConfig;
use crate::state::market::Market;

#[derive(Accounts)]
pub struct UpdateMarket<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [PROTOCOL_SEED],
        bump = config.bump,
        has_one = authority @ M0OracleError::Unauthorized
    )]
    pub config: Account<'info, ProtocolConfig>,

    #[account(
        mut,
        seeds = [MARKET_SEED, market.market_id.as_bytes()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,
}

pub fn handler(ctx: Context<UpdateMarket>, active: Option<bool>) -> Result<()> {
    let cfg = &ctx.accounts.config;
    if cfg.paused {
        return err!(M0OracleError::Paused);
    }

    let m = &mut ctx.accounts.market;
    if let Some(a) = active {
        m.active = a;
    }

    emit!(MarketUpdated {
        market: m.key(),
        market_id: m.market_id.clone(),
        active: m.active,
        updated_at_slot: Clock::get()?.slot,
    });

    Ok(())
}
