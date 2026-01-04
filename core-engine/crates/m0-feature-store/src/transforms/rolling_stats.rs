
use serde_json::json;

pub fn rolling_stats(features: &serde_json::Value) -> serde_json::Value {
    // Placeholder rolling stats.
    json!({
        "rolling": { "mean": null, "std": null },
        "base": features
    })
}
