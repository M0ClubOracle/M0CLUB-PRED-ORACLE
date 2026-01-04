
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalEvent {
    pub market_id: String,
    pub observed_at_ms: u64,
    pub features: serde_json::Value,
    pub quality_flags: u32,
}
