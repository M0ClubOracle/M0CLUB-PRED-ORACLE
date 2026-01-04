
use anchor_lang::prelude::*;
use crate::state::router::{Router, ROUTER_SEED};
use crate::state::vaults::{FeeVault, VAULT_SEED};

// This instruction is a skeleton:
// In production you would route SPL token fees from a vault ATA to destination ATAs.
// For simplicity, this program stores routing config only.
// Fee movement can be implemented by adding token accounts + CPI to token program.

#[derive(Accounts)]
pub struct RouteFees<'info> {
    #[account(
        seeds = [ROUTER_SEED],
        bump = router.bump
    )]
    pub router: Account<'info, Router>,

    #[account(
        seeds = [VAULT_SEED],
        bump = vault.bump
    )]
    pub vault: Account<'info, FeeVault>,
}

pub fn handler(_ctx: Context<RouteFees>) -> Result<()> {
    Ok(())
}
