
use m0_normalizer::{normalize};
use m0_ingestor::stream::schema::RawEvent;
use m0_normalizer::schema::canonical::CanonicalEvent;

pub fn normalize_event(ev: &RawEvent) -> Result<CanonicalEvent, m0_normalizer::error::NormalizeError> {
    normalize(ev)
}
