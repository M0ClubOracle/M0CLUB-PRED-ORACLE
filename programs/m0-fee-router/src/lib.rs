
use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("M0FeeRou7er11111111111111111111111111111");

#[program]
pub mod m0_fee_router {
    use super::*;

    pub fn init_router(ctx: Context<init_router::InitRouter>) -> Result<()> {
        init_router::handler(ctx)
    }

    pub fn set_routes(ctx: Context<set_routes::SetRoutes>, routes: Vec<state::router::Route>) -> Result<()> {
        set_routes::handler(ctx, routes)
    }

    pub fn route_fees(ctx: Context<route_fees::RouteFees>) -> Result<()> {
        route_fees::handler(ctx)
    }
}
