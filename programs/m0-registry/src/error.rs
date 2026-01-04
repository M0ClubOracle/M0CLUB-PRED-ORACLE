
use anchor_lang::prelude::*;

#[error_code]
pub enum M0RegistryError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Already initialized")]
    AlreadyInitialized,
    #[msg("Invalid parameter")]
    InvalidParameter,
}
