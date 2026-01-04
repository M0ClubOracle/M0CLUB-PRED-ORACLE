
use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("M0G0v3rnance111111111111111111111111111");

#[program]
pub mod m0_governance {
    use super::*;

    pub fn init_governor(ctx: Context<init_governor::InitGovernor>, voting_period_slots: u64, quorum_bps: u16, min_delay_slots: u64) -> Result<()> {
        init_governor::handler(ctx, voting_period_slots, quorum_bps, min_delay_slots)
    }

    pub fn set_guardians(ctx: Context<set_guardians::SetGuardians>, guardians: Vec<Pubkey>) -> Result<()> {
        set_guardians::handler(ctx, guardians)
    }

    pub fn propose(ctx: Context<propose::Propose>, actions: Vec<state::governor::Action>) -> Result<()> {
        propose::handler(ctx, actions)
    }

    pub fn vote(ctx: Context<vote::Vote>, support: bool, weight: u64) -> Result<()> {
        vote::handler(ctx, support, weight)
    }

    pub fn execute(ctx: Context<execute::Execute>) -> Result<()> {
        execute::handler(ctx)
    }
}
