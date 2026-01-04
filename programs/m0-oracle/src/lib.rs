
use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;
pub mod verify;

use instructions::*;

declare_id!("M0Orac1e111111111111111111111111111111111");

#[program]
pub mod m0_oracle {
    use super::*;

    pub fn init_protocol(ctx: Context<init_protocol::InitProtocol>, default_reveal_delay_slots: Option<u64>) -> Result<()> {
        init_protocol::handler(ctx, default_reveal_delay_slots)
    }

    pub fn create_market(ctx: Context<create_market::CreateMarket>, market_id: String, domain: state::market::Domain, outcomes: Vec<String>, active: bool) -> Result<()> {
        create_market::handler(ctx, market_id, domain, outcomes, active)
    }

    pub fn update_market(ctx: Context<update_market::UpdateMarket>, active: Option<bool>) -> Result<()> {
        update_market::handler(ctx, active)
    }

    pub fn open_epoch(ctx: Context<open_epoch::OpenEpoch>) -> Result<()> {
        open_epoch::handler(ctx)
    }

    pub fn commit_prediction(ctx: Context<commit_prediction::CommitPrediction>, commit_hash: [u8; 32], reveal_delay_slots: Option<u64>) -> Result<()> {
        commit_prediction::handler(ctx, commit_hash, reveal_delay_slots)
    }

    pub fn reveal_prediction(ctx: Context<reveal_prediction::RevealPrediction>, bundle: state::reveal::BundleReveal, salt: [u8; 32], bundle_bytes: Vec<u8>) -> Result<()> {
        reveal_prediction::handler(ctx, bundle, salt, bundle_bytes)
    }

    pub fn finalize_epoch(ctx: Context<finalize_epoch::FinalizeEpoch>) -> Result<()> {
        finalize_epoch::handler(ctx)
    }

    pub fn rotate_signer_set(ctx: Context<rotate_signer_set::RotateSignerSet>, threshold: u16, pubkeys: Vec<Pubkey>, active: bool) -> Result<()> {
        rotate_signer_set::handler(ctx, threshold, pubkeys, active)
    }

    pub fn set_paused(ctx: Context<pause_resume::SetPaused>, paused: bool) -> Result<()> {
        pause_resume::handler(ctx, paused)
    }

    pub fn upgrade_admin(ctx: Context<upgrade_admin::UpgradeAdmin>, new_authority: Pubkey) -> Result<()> {
        upgrade_admin::handler(ctx, new_authority)
    }
}
