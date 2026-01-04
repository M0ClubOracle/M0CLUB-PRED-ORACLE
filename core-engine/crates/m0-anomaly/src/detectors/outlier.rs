
pub fn is_outlier(x: f64, mean: f64, std: f64, z: f64) -> bool {
    if std <= 0.0 { return false; }
    ((x - mean) / std).abs() > z
}
