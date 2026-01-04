
use anchor_lang::prelude::*;
use crate::error::M0GovernanceError;
use crate::state::governor::{Proposal, GOVERNOR_SEED, PROPOSAL_SEED};

#[derive(Accounts)]
pub struct Vote<'info> {
    pub voter: Signer<'info>,

    #[account(
        mut,
        seeds = [PROPOSAL_SEED, &proposal.proposal_id.to_le_bytes()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
}

pub fn handler(ctx: Context<Vote>, support: bool, weight: u64) -> Result<()> {
    let now = Clock::get()?.slot;
    let p = &mut ctx.accounts.proposal;

    if now > p.voting_ends_at_slot {
        return err!(M0GovernanceError::VotingClosed);
    }
    if p.executed {
        return err!(M0GovernanceError::ProposalExecuted);
    }

    if support {
        p.yes_votes = p.yes_votes.saturating_add(weight);
    } else {
        p.no_votes = p.no_votes.saturating_add(weight);
    }
    Ok(())
}
