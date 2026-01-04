
pub fn brier(p: f64, outcome: bool) -> f64 {
    let y = if outcome { 1.0 } else { 0.0 };
    (p - y).powi(2)
}
