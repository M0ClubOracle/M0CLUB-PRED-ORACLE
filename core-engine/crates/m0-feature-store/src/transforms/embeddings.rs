
use serde_json::json;
use sha2::{Digest, Sha256};

pub fn embed_text(text: &str) -> Vec<f32> {
    // Deterministic pseudo-embedding using sha256.
    let mut h = Sha256::new();
    h.update(text.as_bytes());
    let out = h.finalize();
    out.iter().take(32).map(|b| (*b as f32) / 255.0).collect()
}

pub fn attach_embedding(features: &serde_json::Value, text: &str) -> serde_json::Value {
    json!({
        "embedding": embed_text(text),
        "base": features
    })
}
