
pub fn logloss(p: f64, outcome: bool) -> f64 {
    let p = p.clamp(1e-12, 1.0-1e-12);
    if outcome { -p.ln() } else { -(1.0-p).ln() }
}
