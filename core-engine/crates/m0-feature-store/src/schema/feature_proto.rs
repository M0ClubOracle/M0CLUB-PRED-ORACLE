
// In a production repo this file is generated from feature.proto via prost-build.
// For this skeleton, we keep a small struct and stable JSON encoding.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureRow {
    pub market_id: String,
    pub ts_ms: u64,
    pub features: serde_json::Value,
}
