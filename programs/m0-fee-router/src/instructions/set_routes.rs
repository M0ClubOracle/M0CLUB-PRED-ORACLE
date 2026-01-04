
use anchor_lang::prelude::*;
use crate::error::M0FeeRouterError;
use crate::state::router::{Router, Route, ROUTER_SEED};

#[derive(Accounts)]
pub struct SetRoutes<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [ROUTER_SEED],
        bump = router.bump,
        has_one = authority @ M0FeeRouterError::Unauthorized
    )]
    pub router: Account<'info, Router>,
}

pub fn handler(ctx: Context<SetRoutes>, routes: Vec<Route>) -> Result<()> {
    let mut sum: u32 = 0;
    for r in &routes {
        sum += r.bps as u32;
    }
    if sum != 10_000 {
        return err!(M0FeeRouterError::InvalidParameter);
    }
    ctx.accounts.router.routes = routes;
    Ok(())
}
