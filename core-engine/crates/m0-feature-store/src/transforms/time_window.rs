
use serde_json::json;

pub fn windowize(features: &serde_json::Value, window_ms: u64) -> serde_json::Value {
    json!({
        "window_ms": window_ms,
        "base": features
    })
}
