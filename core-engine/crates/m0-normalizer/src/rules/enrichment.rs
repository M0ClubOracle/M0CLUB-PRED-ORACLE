
use serde_json::json;
use crate::schema::raw::RawEvent;

pub fn enrich(ev: &RawEvent) -> serde_json::Value {
    // Add basic enrichment metadata.
    json!({
        "source": format!("{:?}", ev.source),
        "observed_at_ms": ev.observed_at_ms,
        "payload": ev.payload
    })
}
