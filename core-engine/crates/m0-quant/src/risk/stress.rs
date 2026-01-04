
pub fn stress(p: f64, shock: f64) -> f64 {
    (p + shock).clamp(0.0, 1.0)
}
