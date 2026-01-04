
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureVector {
    pub market_id: String,
    pub ts_ms: u64,
    pub features: serde_json::Value,
}
