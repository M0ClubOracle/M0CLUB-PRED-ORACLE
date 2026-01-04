
pub mod error;
pub mod rules;
pub mod schema;

use schema::raw::RawEvent;
use schema::canonical::CanonicalEvent;

pub fn normalize(ev: &RawEvent) -> Result<CanonicalEvent, error::NormalizeError> {
    rules::canonicalize::canonicalize(ev)
}
