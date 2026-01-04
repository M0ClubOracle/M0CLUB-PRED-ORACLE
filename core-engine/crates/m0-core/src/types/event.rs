
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineTick {
    pub tick_index: u32,
    pub observed_at_ms: u64,
}
