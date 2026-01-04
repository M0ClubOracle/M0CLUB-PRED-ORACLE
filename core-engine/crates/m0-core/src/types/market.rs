
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDef {
    pub market_id: String,
    pub outcomes: Vec<String>,
    pub domain: String,
    pub cadence_ms: u32,
}
