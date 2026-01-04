
use chrono::{DateTime, Utc};

pub fn now_utc() -> DateTime<Utc> {
    Utc::now()
}

pub fn now_ms() -> u64 {
    let dt = now_utc();
    dt.timestamp_millis().max(0) as u64
}
