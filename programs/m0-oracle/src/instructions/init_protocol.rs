
use anchor_lang::prelude::*;
use crate::events::*;
use crate::state::config::ProtocolConfig;
use crate::constants::*;
use crate::error::M0OracleError;

#[derive(Accounts)]
pub struct InitProtocol<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = ProtocolConfig::LEN,
        seeds = [PROTOCOL_SEED],
        bump
    )]
    pub config: Account<'info, ProtocolConfig>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitProtocol>, default_reveal_delay_slots: Option<u64>) -> Result<()> {
    let cfg = &mut ctx.accounts.config;
    if cfg.initialized {
        return err!(M0OracleError::AlreadyInitialized);
    }

    cfg.initialized = true;
    cfg.authority = ctx.accounts.authority.key();
    cfg.paused = false;
    cfg.next_market_nonce = 0;
    cfg.next_signer_set_id = 1;
    cfg.default_reveal_delay_slots = default_reveal_delay_slots.unwrap_or(DEFAULT_REVEAL_DELAY_SLOTS);
    cfg.bump = *ctx.bumps.get("config").unwrap();

    emit!(ProtocolInitialized {
        authority: cfg.authority,
        created_at_slot: Clock::get()?.slot,
    });

    Ok(())
}
