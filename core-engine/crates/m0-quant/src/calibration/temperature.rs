
pub fn temperature_scale(logit: f64, t: f64) -> f64 {
    let t = t.max(1e-6);
    1.0 / (1.0 + (-logit / t).exp())
}
