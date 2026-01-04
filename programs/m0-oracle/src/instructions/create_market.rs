
use anchor_lang::prelude::*;
use crate::constants::*;
use crate::events::*;
use crate::error::M0OracleError;
use crate::state::config::ProtocolConfig;
use crate::state::market::{Market, Domain};

#[derive(Accounts)]
#[instruction(market_id: String)]
pub struct CreateMarket<'info> {
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
        space = Market::len_with(MAX_OUTCOMES, market_id.len().max(1)),
        seeds = [MARKET_SEED, market_id.as_bytes()],
        bump
    )]
    pub market: Account<'info, Market>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateMarket>, market_id: String, domain: Domain, outcomes: Vec<String>, active: bool) -> Result<()> {
    let cfg = &mut ctx.accounts.config;
    if cfg.paused {
        return err!(M0OracleError::Paused);
    }

    Market::validate_ids(&market_id, &outcomes)?;

    let m = &mut ctx.accounts.market;
    m.market_id = market_id.clone();
    m.domain = domain;
    m.active = active;
    m.outcomes = outcomes;
    m.current_epoch_id = 0;
    m.last_sequence = 0;
    m.bump = *ctx.bumps.get("market").unwrap();

    cfg.next_market_nonce = cfg.next_market_nonce.saturating_add(1);

    emit!(MarketCreated {
        market: m.key(),
        market_id,
        created_at_slot: Clock::get()?.slot,
    });

    Ok(())
}
