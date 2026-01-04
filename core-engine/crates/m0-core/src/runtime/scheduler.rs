
use tokio::time::{Duration, Interval};
use crate::types::event::EngineTick;
use m0_common::time::now_ms;

pub fn tick_interval(tick_ms: u64) -> Interval {
    tokio::time::interval(Duration::from_millis(tick_ms.max(50)))
}

pub fn current_tick(index: u32) -> EngineTick {
    EngineTick { tick_index: index, observed_at_ms: now_ms() }
}
