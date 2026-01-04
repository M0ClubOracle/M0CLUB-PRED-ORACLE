
use crate::error::AnomalyError;

pub fn enforce_probability_bounds(p: f64) -> Result<(), AnomalyError> {
    if !(0.0..=1.0).contains(&p) {
        return Err(AnomalyError::Detected("probability out of bounds".into()));
    }
    Ok(())
}
