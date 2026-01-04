
use anchor_lang::prelude::*;
use crate::state::router::{Router, ROUTER_SEED};
use crate::state::vaults::{FeeVault, VAULT_SEED};

#[derive(Accounts)]
pub struct InitRouter<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = Router::len_with(0),
        seeds = [ROUTER_SEED],
        bump
    )]
    pub router: Account<'info, Router>,

    #[account(
        init,
        payer = authority,
        space = FeeVault::LEN,
        seeds = [VAULT_SEED],
        bump
    )]
    pub vault: Account<'info, FeeVault>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitRouter>) -> Result<()> {
    let r = &mut ctx.accounts.router;
    r.authority = ctx.accounts.authority.key();
    r.routes = vec![];
    r.bump = *ctx.bumps.get("router").unwrap();

    let v = &mut ctx.accounts.vault;
    v.router = r.key();
    v.bump = *ctx.bumps.get("vault").unwrap();

    Ok(())
}
