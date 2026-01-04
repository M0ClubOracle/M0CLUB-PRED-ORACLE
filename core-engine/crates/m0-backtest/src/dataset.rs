
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetRow {
    pub market_id: String,
    pub observed_at_ms: u64,
    pub outcome: bool,
    pub p: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Dataset {
    pub rows: Vec<DatasetRow>,
}
