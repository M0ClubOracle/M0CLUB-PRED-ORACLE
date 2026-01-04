
// Auto-curated shared types for M0Club SDKs.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type MarketId = String;
pub type EpochId = u64;
pub type ConfidenceInterval = [f64; 2];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutcomeProb {
    pub p: f64,
    pub ci: ConfidenceInterval,
}

pub type PredictionPayload = BTreeMap<String, OutcomeProb>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    pub market_id: MarketId,
    pub epoch_id: EpochId,
    pub outcomes: PredictionPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub market_id: MarketId,
    pub domain: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Epoch {
    pub epoch_id: EpochId,
    pub market_id: MarketId,
    pub state: String,
}
