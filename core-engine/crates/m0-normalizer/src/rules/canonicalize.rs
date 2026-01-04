
use crate::schema::raw::RawEvent;
use crate::schema::canonical::CanonicalEvent;
use crate::rules::{validation, enrichment};

pub fn canonicalize(ev: &RawEvent) -> Result<CanonicalEvent, crate::error::NormalizeError> {
    validation::validate(ev)?;
    Ok(CanonicalEvent {
        market_id: ev.market_id.clone(),
        observed_at_ms: ev.observed_at_ms,
        features: enrichment::enrich(ev),
        quality_flags: 0,
    })
}
