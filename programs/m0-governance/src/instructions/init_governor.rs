
use anchor_lang::prelude::*;
use crate::state::governor::{Governor, GOVERNOR_SEED};
use crate::state::timelock::{Timelock, TIMELOCK_SEED};

#[derive(Accounts)]
pub struct InitGovernor<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = Governor::len_with(0),
        seeds = [GOVERNOR_SEED],
        bump
    )]
    pub governor: Account<'info, Governor>,

    #[account(
        init,
        payer = authority,
        space = Timelock::LEN,
        seeds = [TIMELOCK_SEED],
        bump
    )]
    pub timelock: Account<'info, Timelock>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitGovernor>, voting_period_slots: u64, quorum_bps: u16, min_delay_slots: u64) -> Result<()> {
    let g = &mut ctx.accounts.governor;
    g.authority = ctx.accounts.authority.key();
    g.guardians = vec![];
    g.voting_period_slots = voting_period_slots.max(1);
    g.quorum_bps = quorum_bps.min(10_000);
    g.proposal_count = 0;
    g.bump = *ctx.bumps.get("governor").unwrap();

    let t = &mut ctx.accounts.timelock;
    t.governor = g.key();
    t.min_delay_slots = min_delay_slots;
    t.bump = *ctx.bumps.get("timelock").unwrap();

    Ok(())
}
