
pub mod bayes;
pub mod calibration;
pub mod confidence;
pub mod models;
pub mod risk;
pub mod scoring;
pub mod utils;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbabilityPoint {
    pub outcome_id: String,
    pub p: f64,
    pub ci_low: f64,
    pub ci_high: f64,
    pub ci_level: f64,
    pub quality_flags: u32,
}
