
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct RateLimiter {
    capacity: u32,
    refill_per_sec: u32,
    tokens: u32,
    last: Instant,
}

impl RateLimiter {
    pub fn new(capacity: u32, refill_per_sec: u32) -> Self {
        Self { capacity, refill_per_sec, tokens: capacity, last: Instant::now() }
    }

    pub fn try_take(&mut self, n: u32) -> bool {
        self.refill();
        if self.tokens >= n {
            self.tokens -= n;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let elapsed = self.last.elapsed();
        if elapsed < Duration::from_millis(50) {
            return;
        }
        let add = (elapsed.as_secs_f64() * self.refill_per_sec as f64) as u32;
        if add > 0 {
            self.tokens = (self.tokens + add).min(self.capacity);
            self.last = Instant::now();
        }
    }
}
