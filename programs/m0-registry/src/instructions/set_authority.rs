
use anchor_lang::prelude::*;
use crate::events::*;
use crate::state::registry::{Registry, REGISTRY_SEED};
use crate::error::M0RegistryError;

#[derive(Accounts)]
pub struct SetAuthority<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [REGISTRY_SEED],
        bump = registry.bump,
        has_one = authority @ M0RegistryError::Unauthorized
    )]
    pub registry: Account<'info, Registry>,
}

pub fn handler(ctx: Context<SetAuthority>, new_authority: Pubkey) -> Result<()> {
    let r = &mut ctx.accounts.registry;
    r.authority = new_authority;
    emit!(AuthoritySet { new_authority });
    Ok(())
}
