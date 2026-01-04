
pub fn drift_score(current: f64, baseline: f64) -> f64 {
    (current - baseline).abs()
}
