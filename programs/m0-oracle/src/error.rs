
use anchor_lang::prelude::*;

#[error_code]
pub enum M0OracleError {
    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Protocol already initialized")]
    AlreadyInitialized,

    #[msg("Market already exists")]
    MarketAlreadyExists,

    #[msg("Market not active")]
    MarketNotActive,

    #[msg("Epoch not open")]
    EpochNotOpen,

    #[msg("Epoch already open")]
    EpochAlreadyOpen,

    #[msg("Invalid market id")]
    InvalidMarketId,

    #[msg("Invalid outcome id")]
    InvalidOutcomeId,

    #[msg("Invalid probability scale")]
    InvalidProbabilityScale,

    #[msg("Commit not found")]
    CommitNotFound,

    #[msg("Commit already revealed")]
    CommitAlreadyRevealed,

    #[msg("Reveal mismatch")]
    RevealMismatch,

    #[msg("Reveal too early")]
    RevealTooEarly,

    #[msg("Replay protection triggered")]
    ReplayViolation,

    #[msg("Signer set not active")]
    SignerSetNotActive,

    #[msg("Invalid threshold")]
    InvalidThreshold,

    #[msg("Bundle hash mismatch")]
    BundleHashMismatch,

    #[msg("Invalid instruction sysvar")]
    InvalidInstructionsSysvar,

    #[msg("Signature verification failed")]
    SignatureVerificationFailed,

    #[msg("Paused")]
    Paused,

    #[msg("Invalid parameter")]
    InvalidParameter,
}
