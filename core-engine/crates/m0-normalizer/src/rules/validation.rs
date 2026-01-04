
use crate::schema::raw::RawEvent;
use crate::error::NormalizeError;

pub fn validate(ev: &RawEvent) -> Result<(), NormalizeError> {
    if ev.market_id.trim().is_empty() {
        return Err(NormalizeError::Invalid("market_id empty".into()));
    }
    Ok(())
}
