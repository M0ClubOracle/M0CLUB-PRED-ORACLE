
pub const PROTOCOL_SEED: &[u8] = b"protocol";
pub const MARKET_SEED: &[u8] = b"market";
pub const EPOCH_SEED: &[u8] = b"epoch";
pub const SIGNER_SET_SEED: &[u8] = b"signer_set";
pub const COMMIT_SEED: &[u8] = b"commit";
pub const AUDIT_SEED: &[u8] = b"audit";

pub const PROB_SCALE: u64 = 1_000_000_000;
pub const MAX_OUTCOMES: usize = 16;
pub const MAX_MARKET_ID_LEN: usize = 64;
pub const MAX_OUTCOME_ID_LEN: usize = 64;

pub const DEFAULT_REVEAL_DELAY_SLOTS: u64 = 10;
