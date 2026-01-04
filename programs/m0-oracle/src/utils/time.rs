
use anchor_lang::prelude::*;

pub fn now_slot() -> u64 {
    Clock::get().map(|c| c.slot).unwrap_or(0)
}
