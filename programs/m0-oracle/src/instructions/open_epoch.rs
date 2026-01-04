
use anchor_lang::prelude::*;
use crate::constants::*;
use crate::events::*;
use crate::error::M0OracleError;
use crate::state::config::ProtocolConfig;
use crate::state::epoch::Epoch;
use crate::state::market::Market;

#[derive(Accounts)]
pub struct OpenEpoch<'info> {
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

    #[account(
        init,
        payer = authority,
        space = Epoch::LEN,
        seeds = [EPOCH_SEED, market.key().as_ref(), &market.current_epoch_id.saturating_add(1).to_le_bytes()],
        bump
    )]
    pub epoch: Account<'info, Epoch>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<OpenEpoch>) -> Result<()> {
    let cfg = &ctx.accounts.config;
    if cfg.paused {
        return err!(M0OracleError::Paused);
    }

    let m = &mut ctx.accounts.market;
    if !m.active {
        return err!(M0OracleError::MarketNotActive);
    }

    let next_epoch_id = m.current_epoch_id.saturating_add(1);

    let e = &mut ctx.accounts.epoch;
    e.market = m.key();
    e.epoch_id = next_epoch_id;
    e.open = true;
    e.opened_at_slot = Clock::get()?.slot;
    e.finalized_at_slot = 0;
    e.publish_sequence = 0;
    e.bump = *ctx.bumps.get("epoch").unwrap();

    m.current_epoch_id = next_epoch_id;

    emit!(EpochOpened {
        epoch: e.key(),
        market: m.key(),
        epoch_id: e.epoch_id,
        opened_at_slot: e.opened_at_slot,
    });

    Ok(())
}
