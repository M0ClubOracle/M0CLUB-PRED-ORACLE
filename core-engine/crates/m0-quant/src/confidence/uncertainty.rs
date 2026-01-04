
pub fn entropy(p: f64) -> f64 {
    let p = p.clamp(1e-12, 1.0-1e-12);
    -(p * p.ln() + (1.0 - p) * (1.0 - p).ln())
}
