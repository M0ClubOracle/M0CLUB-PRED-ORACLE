
use anchor_lang::prelude::*;
use crate::error::M0GovernanceError;
use crate::state::governor::{Governor, GOVERNOR_SEED};

#[derive(Accounts)]
pub struct SetGuardians<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [GOVERNOR_SEED],
        bump = governor.bump,
        has_one = authority @ M0GovernanceError::Unauthorized
    )]
    pub governor: Account<'info, Governor>,
}

pub fn handler(ctx: Context<SetGuardians>, guardians: Vec<Pubkey>) -> Result<()> {
    ctx.accounts.governor.guardians = guardians;
    Ok(())
}
