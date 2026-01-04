
use anchor_lang::prelude::*;
use crate::constants::*;
use crate::events::*;
use crate::error::M0OracleError;
use crate::state::commit::CommitRecord;
use crate::state::config::ProtocolConfig;
use crate::state::epoch::Epoch;
use crate::state::market::Market;

#[derive(Accounts)]
pub struct CommitPrediction<'info> {
    #[account(mut)]
    pub committer: Signer<'info>,

    #[account(
        seeds = [PROTOCOL_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, ProtocolConfig>,

    #[account(
        seeds = [MARKET_SEED, market.market_id.as_bytes()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        seeds = [EPOCH_SEED, market.key().as_ref(), &epoch.epoch_id.to_le_bytes()],
        bump = epoch.bump
    )]
    pub epoch: Account<'info, Epoch>,

    #[account(
        init,
        payer = committer,
        space = CommitRecord::LEN,
        seeds = [COMMIT_SEED, epoch.key().as_ref(), committer.key().as_ref()],
        bump
    )]
    pub commit: Account<'info, CommitRecord>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CommitPrediction>, commit_hash: [u8; 32], reveal_delay_slots: Option<u64>) -> Result<()> {
    let cfg = &ctx.accounts.config;
    if cfg.paused {
        return err!(M0OracleError::Paused);
    }

    if !ctx.accounts.market.active {
        return err!(M0OracleError::MarketNotActive);
    }
    if !ctx.accounts.epoch.open {
        return err!(M0OracleError::EpochNotOpen);
    }

    let delay = reveal_delay_slots.unwrap_or(cfg.default_reveal_delay_slots);
    let now = Clock::get()?.slot;
    let reveal_after = now.saturating_add(delay);

    let c = &mut ctx.accounts.commit;
    c.market = ctx.accounts.market.key();
    c.epoch = ctx.accounts.epoch.key();
    c.committer = ctx.accounts.committer.key();
    c.commit_hash = commit_hash;
    c.reveal_after_slot = reveal_after;
    c.revealed = false;
    c.bump = *ctx.bumps.get("commit").unwrap();

    emit!(PredictionCommitted {
        market: c.market,
        epoch: c.epoch,
        committer: c.committer,
        commit_hash: c.commit_hash,
        reveal_after_slot: reveal_after,
    });

    Ok(())
}
