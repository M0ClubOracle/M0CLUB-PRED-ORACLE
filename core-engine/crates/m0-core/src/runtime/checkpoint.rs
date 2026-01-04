
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Checkpoint {
    pub last_tick_index: u32,
    pub last_bundle_hash_hex: Option<String>,
}
