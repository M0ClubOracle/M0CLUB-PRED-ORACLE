
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionPoint {
    pub outcome_id: String,
    pub p: f64,
}
