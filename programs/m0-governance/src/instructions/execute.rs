
use anchor_lang::prelude::*;
use crate::error::M0GovernanceError;
use crate::state::governor::{Proposal, PROPOSAL_SEED};
use crate::state::timelock::{Timelock, TIMELOCK_SEED};
use crate::state::governor::{Governor, GOVERNOR_SEED};

#[derive(Accounts)]
pub struct Execute<'info> {
    pub executor: Signer<'info>,

    #[account(
        seeds = [GOVERNOR_SEED],
        bump = governor.bump
    )]
    pub governor: Account<'info, Governor>,

    #[account(
        seeds = [TIMELOCK_SEED],
        bump = timelock.bump
    )]
    pub timelock: Account<'info, Timelock>,

    #[account(
        mut,
        seeds = [PROPOSAL_SEED, &proposal.proposal_id.to_le_bytes()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
}

pub fn handler(ctx: Context<Execute>) -> Result<()> {
    let now = Clock::get()?.slot;
    let g = &ctx.accounts.governor;
    let t = &ctx.accounts.timelock;
    let p = &mut ctx.accounts.proposal;

    if p.executed {
        return err!(M0GovernanceError::ProposalExecuted);
    }

    if now <= p.voting_ends_at_slot.saturating_add(t.min_delay_slots) {
        return err!(M0GovernanceError::VotingClosed);
    }

    // Quorum check (simple): yes_votes must be >= quorum bps of total votes.
    let total = p.yes_votes.saturating_add(p.no_votes).max(1);
    let yes_bps = (p.yes_votes.saturating_mul(10_000)) / total;
    if yes_bps < g.quorum_bps as u64 {
        return err!(M0GovernanceError::InvalidParameter);
    }

    // Skeleton: in production, you would CPI invoke each action.
    // Anchor CPI requires accounts; a generic executor is more complex.
    // This program marks executed only.
    p.executed = true;

    Ok(())
}
