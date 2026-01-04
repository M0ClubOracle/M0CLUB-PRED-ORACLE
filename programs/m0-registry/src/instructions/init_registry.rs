
use anchor_lang::prelude::*;
use crate::events::*;
use crate::state::registry::{Registry, REGISTRY_SEED};
use crate::error::M0RegistryError;

#[derive(Accounts)]
pub struct InitRegistry<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = Registry::LEN,
        seeds = [REGISTRY_SEED],
        bump
    )]
    pub registry: Account<'info, Registry>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitRegistry>) -> Result<()> {
    let r = &mut ctx.accounts.registry;
    if r.initialized {
        return err!(M0RegistryError::AlreadyInitialized);
    }
    r.initialized = true;
    r.authority = ctx.accounts.authority.key();
    r.market_count = 0;
    r.bump = *ctx.bumps.get("registry").unwrap();

    emit!(RegistryInitialized {
        authority: r.authority,
        created_at_slot: Clock::get()?.slot,
    });

    Ok(())
}
