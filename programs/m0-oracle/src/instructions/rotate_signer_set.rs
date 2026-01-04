
use anchor_lang::prelude::*;
use crate::error::M0OracleError;
use crate::events::*;
use crate::state::config::ProtocolConfig;
use crate::state::signer_set::SignerSet;
use crate::constants::*;

#[derive(Accounts)]
pub struct RotateSignerSet<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [PROTOCOL_SEED],
        bump = config.bump,
        has_one = authority @ M0OracleError::Unauthorized
    )]
    pub config: Account<'info, ProtocolConfig>,

    #[account(
        init,
        payer = authority,
        space = SignerSet::len_with(pubkeys.len()),
        seeds = [SIGNER_SET_SEED, &config.next_signer_set_id.to_le_bytes()],
        bump
    )]
    pub signer_set: Account<'info, SignerSet>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RotateSignerSet>, threshold: u16, pubkeys: Vec<Pubkey>, active: bool) -> Result<()> {
    let cfg = &mut ctx.accounts.config;
    if cfg.paused {
        return err!(M0OracleError::Paused);
    }

    SignerSet::validate(threshold, pubkeys.len())?;

    let ss_id = cfg.next_signer_set_id;
    cfg.next_signer_set_id = cfg.next_signer_set_id.saturating_add(1);

    let ss = &mut ctx.accounts.signer_set;
    ss.signer_set_id = ss_id;
    ss.threshold = threshold;
    ss.pubkeys = pubkeys;
    ss.active = active;
    ss.created_at_slot = Clock::get()?.slot;
    ss.bump = *ctx.bumps.get("signer_set").unwrap();

    emit!(SignerSetRotated {
        signer_set: ss.key(),
        signer_set_id: ss.signer_set_id,
        threshold: ss.threshold,
        active: ss.active,
    });

    Ok(())
}
