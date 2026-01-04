
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceKind {
    Solana,
    Sports,
    Politics,
    Macro,
    Webhook,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawEvent {
    pub source: SourceKind,
    pub market_id: String,
    pub observed_at_ms: u64,
    pub payload: serde_json::Value,
    pub dedupe_key: String,
}
