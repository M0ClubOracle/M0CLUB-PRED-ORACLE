
use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::M0OracleError;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Domain {
    Sports,
    Politics,
    Macro,
    Crypto,
    Custom,
}

#[account]
pub struct Market {
    pub market_id: String,
    pub domain: Domain,
    pub active: bool,
    pub outcomes: Vec<String>,
    pub current_epoch_id: u64,
    pub last_sequence: u64,
    pub bump: u8,
}

impl Market {
    pub fn validate_ids(market_id: &str, outcomes: &[String]) -> Result<()> {
        if market_id.is_empty() || market_id.len() > MAX_MARKET_ID_LEN {
            return err!(M0OracleError::InvalidMarketId);
        }
        if outcomes.is_empty() || outcomes.len() > MAX_OUTCOMES {
            return err!(M0OracleError::InvalidParameter);
        }
        for o in outcomes {
            if o.is_empty() || o.len() > MAX_OUTCOME_ID_LEN {
                return err!(M0OracleError::InvalidOutcomeId);
            }
        }
        Ok(())
    }

    pub fn len_with(outcome_count: usize, market_id_len: usize) -> usize {
        // Anchor account discriminator (8) is included by the runtime.
        // LEN here is for space allocation: 8 + fields.
        // market_id: 4 + bytes
        // domain: 1
        // active: 1
        // outcomes: 4 + outcome_count*(4 + bytes)
        // current_epoch_id: 8
        // last_sequence: 8
        // bump: 1
        8 + 4 + market_id_len + 1 + 1 + 4 + outcome_count * (4 + MAX_OUTCOME_ID_LEN) + 8 + 8 + 1
    }
}
