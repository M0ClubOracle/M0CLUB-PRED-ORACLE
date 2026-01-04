
pub fn var95(returns: &mut [f64]) -> f64 {
    if returns.is_empty() { return 0.0; }
    returns.sort_by(|a,b| a.partial_cmp(b).unwrap());
    let idx = ((returns.len() as f64) * 0.05).floor() as usize;
    returns[idx.min(returns.len()-1)]
}
