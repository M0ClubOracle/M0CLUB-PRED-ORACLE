
use serde_json::json;

pub fn domain_features(domain: &str, features: &serde_json::Value) -> serde_json::Value {
    json!({
        "domain": domain,
        "base": features
    })
}
