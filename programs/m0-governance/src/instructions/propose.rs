
use anchor_lang::prelude::*;
use crate::error::M0GovernanceError;
use crate::state::governor::{Governor, Proposal, Action, GOVERNOR_SEED, PROPOSAL_SEED};

const MAX_ACTIONS: usize = 8;
const MAX_ACTION_ACCOUNTS: usize = 16;
const MAX_ACTION_DATA: usize = 512;

#[derive(Accounts)]
pub struct Propose<'info> {
    #[account(mut)]
    pub proposer: Signer<'info>,

    #[account(
        mut,
        seeds = [GOVERNOR_SEED],
        bump = governor.bump
    )]
    pub governor: Account<'info, Governor>,

    #[account(
        init,
        payer = proposer,
        space = Proposal::len_with(MAX_ACTIONS, MAX_ACTION_DATA, MAX_ACTION_ACCOUNTS),
        seeds = [PROPOSAL_SEED, &governor.proposal_count.saturating_add(1).to_le_bytes()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Propose>, actions: Vec<Action>) -> Result<()> {
    if actions.is_empty() || actions.len() > MAX_ACTIONS {
        return err!(M0GovernanceError::InvalidParameter);
    }

    let g = &mut ctx.accounts.governor;
    let proposal_id = g.proposal_count.saturating_add(1);
    g.proposal_count = proposal_id;

    let now = Clock::get()?.slot;
    let ends = now.saturating_add(g.voting_period_slots);

    let p = &mut ctx.accounts.proposal;
    p.governor = g.key();
    p.proposer = ctx.accounts.proposer.key();
    p.proposal_id = proposal_id;
    p.created_at_slot = now;
    p.voting_ends_at_slot = ends;
    p.yes_votes = 0;
    p.no_votes = 0;
    p.executed = false;
    p.actions = actions;
    p.bump = *ctx.bumps.get("proposal").unwrap();

    Ok(())
}
