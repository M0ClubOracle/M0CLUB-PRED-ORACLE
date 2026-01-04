
use anchor_lang::prelude::*;

#[error_code]
pub enum M0GovernanceError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Invalid parameter")]
    InvalidParameter,
    #[msg("Proposal not found")]
    ProposalNotFound,
    #[msg("Proposal already executed")]
    ProposalExecuted,
    #[msg("Voting closed")]
    VotingClosed,
}
