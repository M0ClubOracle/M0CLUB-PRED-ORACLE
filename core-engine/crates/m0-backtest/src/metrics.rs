
use crate::dataset::Dataset;
use m0_quant::scoring::{brier::brier, logloss::logloss};

#[derive(Debug, Default, Clone)]
pub struct Metrics {
    pub brier_mean: f64,
    pub logloss_mean: f64,
}

pub fn compute(ds: &Dataset) -> Metrics {
    if ds.rows.is_empty() { return Metrics::default(); }
    let mut b = 0.0;
    let mut l = 0.0;
    for r in &ds.rows {
        b += brier(r.p, r.outcome);
        l += logloss(r.p, r.outcome);
    }
    Metrics { brier_mean: b / ds.rows.len() as f64, logloss_mean: l / ds.rows.len() as f64 }
}
