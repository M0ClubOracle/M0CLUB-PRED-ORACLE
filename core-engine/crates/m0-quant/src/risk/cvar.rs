
pub fn cvar95(returns: &mut [f64]) -> f64 {
    if returns.is_empty() { return 0.0; }
    returns.sort_by(|a,b| a.partial_cmp(b).unwrap());
    let cutoff = ((returns.len() as f64) * 0.05).floor() as usize;
    let slice = &returns[..cutoff.max(1)];
    slice.iter().sum::<f64>() / slice.len() as f64
}
