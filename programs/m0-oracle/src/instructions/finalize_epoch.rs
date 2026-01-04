
use anchor_lang::prelude::*;
use crate::error::M0OracleError;
use crate::events::*;
use crate::state::config::ProtocolConfig;
use crate::state::epoch::Epoch;
use crate::state::market::Market;

#[derive(Accounts)]
pub struct FinalizeEpoch<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [crate::constants::PROTOCOL_SEED],
        bump = config.bump,
        has_one = authority @ M0OracleError::Unauthorized
    )]
    pub config: Account<'info, ProtocolConfig>,

    #[account(
        seeds = [crate::constants::MARKET_SEED, market.market_id.as_bytes()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        seeds = [crate::constants::EPOCH_SEED, market.key().as_ref(), &epoch.epoch_id.to_le_bytes()],
        bump = epoch.bump
    )]
    pub epoch: Account<'info, Epoch>,
}

pub fn handler(ctx: Context<FinalizeEpoch>) -> Result<()> {
    let cfg = &ctx.accounts.config;
    if cfg.paused {
        return err!(M0OracleError::Paused);
    }

    let e = &mut ctx.accounts.epoch;
    if !e.open {
        return err!(M0OracleError::EpochNotOpen);
    }

    e.open = false;
    e.finalized_at_slot = Clock::get()?.slot;

    emit!(EpochFinalized {
        epoch: e.key(),
        market: ctx.accounts.market.key(),
        epoch_id: e.epoch_id,
        finalized_at_slot: e.finalized_at_slot,
    });

    Ok(())
}
