
use anchor_lang::prelude::*;
use crate::error::M0OracleError;
use crate::events::*;
use crate::state::audit::AuditLog;
use crate::state::commit::CommitRecord;
use crate::state::config::ProtocolConfig;
use crate::state::epoch::Epoch;
use crate::state::market::Market;
use crate::state::reveal::BundleReveal;
use crate::state::signer_set::SignerSet;
use crate::utils::hashing::{hash_commit, hash_bundle_content, hash_signature_message};
use crate::verify::signature::verify_threshold_signatures_placeholder;

#[derive(Accounts)]
pub struct RevealPrediction<'info> {
    #[account(mut)]
    pub revealer: Signer<'info>,

    #[account(
        seeds = [crate::constants::PROTOCOL_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, ProtocolConfig>,

    #[account(
        seeds = [crate::constants::MARKET_SEED, market.market_id.as_bytes()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        seeds = [crate::constants::EPOCH_SEED, market.key().as_ref(), &epoch.epoch_id.to_le_bytes()],
        bump = epoch.bump
    )]
    pub epoch: Account<'info, Epoch>,

    #[account(
        mut,
        seeds = [crate::constants::COMMIT_SEED, epoch.key().as_ref(), revealer.key().as_ref()],
        bump = commit.bump
    )]
    pub commit: Account<'info, CommitRecord>,

    #[account(
        seeds = [crate::constants::SIGNER_SET_SEED, &signer_set.signer_set_id.to_le_bytes()],
        bump = signer_set.bump
    )]
    pub signer_set: Account<'info, SignerSet>,

    #[account(
        init_if_needed,
        payer = revealer,
        space = AuditLog::LEN,
        seeds = [crate::constants::AUDIT_SEED, epoch.key().as_ref()],
        bump
    )]
    pub audit: Account<'info, AuditLog>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RevealPrediction>, bundle: BundleReveal, salt: [u8; 32], bundle_bytes: Vec<u8>) -> Result<()> {
    let cfg = &ctx.accounts.config;
    if cfg.paused {
        return err!(M0OracleError::Paused);
    }

    if !ctx.accounts.market.active {
        return err!(M0OracleError::MarketNotActive);
    }
    if !ctx.accounts.epoch.open {
        return err!(M0OracleError::EpochNotOpen);
    }

    let c = &mut ctx.accounts.commit;
    if c.revealed {
        return err!(M0OracleError::CommitAlreadyRevealed);
    }

    let now = Clock::get()?.slot;
    if now < c.reveal_after_slot {
        return err!(M0OracleError::RevealTooEarly);
    }

    // 1) compute content hash
    let content_hash = hash_bundle_content(&bundle_bytes);

    // 2) verify commit preimage matches
    let expected_commit = hash_commit(&content_hash, &salt);
    if expected_commit != c.commit_hash {
        return err!(M0OracleError::RevealMismatch);
    }

    // 3) replay protection: advance epoch sequence monotonically
    let e = &mut ctx.accounts.epoch;
    let next_seq = e.publish_sequence.saturating_add(1);
    e.publish_sequence = next_seq;

    // 4) verify signer set (placeholder in this skeleton)
    let ss = &ctx.accounts.signer_set;
    if !ss.active {
        return err!(M0OracleError::SignerSetNotActive);
    }
    SignerSet::validate(ss.threshold, ss.pubkeys.len())?;

    let sig_msg = hash_signature_message(&content_hash, bundle.signer_set_id, bundle.publish_epoch_id, next_seq);
    verify_threshold_signatures_placeholder(&sig_msg, &ss.pubkeys, ss.threshold)?;

    // 5) write audit
    let audit = &mut ctx.accounts.audit;
    audit.market = ctx.accounts.market.key();
    audit.epoch = ctx.accounts.epoch.key();
    audit.last_bundle_hash = content_hash;
    audit.last_sequence = next_seq;
    audit.last_revealed_at_slot = now;
    audit.bump = *ctx.bumps.get("audit").unwrap();

    // 6) mark revealed
    c.revealed = true;

    // 7) update market last_sequence
    let m = &mut ctx.accounts.market;
    m.last_sequence = next_seq;

    emit!(PredictionRevealed {
        market: m.key(),
        epoch: e.key(),
        revealer: ctx.accounts.revealer.key(),
        bundle_hash: content_hash,
        sequence: next_seq,
    });

    Ok(())
}
