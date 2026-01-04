
use anchor_lang::prelude::*;
use crate::error::M0OracleError;
use crate::state::config::ProtocolConfig;
use crate::constants::*;

#[derive(Accounts)]
pub struct UpgradeAdmin<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [PROTOCOL_SEED],
        bump = config.bump,
        has_one = authority @ M0OracleError::Unauthorized
    )]
    pub config: Account<'info, ProtocolConfig>,
}

pub fn handler(ctx: Context<UpgradeAdmin>, new_authority: Pubkey) -> Result<()> {
    let cfg = &mut ctx.accounts.config;
    cfg.authority = new_authority;
    Ok(())
}
