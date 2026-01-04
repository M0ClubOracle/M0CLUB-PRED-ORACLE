
pub fn average(ps: &[f64]) -> f64 {
    if ps.is_empty() { return 0.0; }
    ps.iter().copied().sum::<f64>() / ps.len() as f64
}
