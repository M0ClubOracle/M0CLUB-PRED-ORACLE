
use anchor_lang::prelude::*;

#[error_code]
pub enum M0FeeRouterError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Invalid parameter")]
    InvalidParameter,
    #[msg("Insufficient funds")]
    InsufficientFunds,
}
