
pub fn wilson_ci(p: f64, n: f64, z: f64) -> (f64, f64) {
    if n <= 0.0 { return (0.0, 1.0); }
    let denom = 1.0 + z*z/n;
    let center = (p + z*z/(2.0*n)) / denom;
    let half = (z/denom) * ((p*(1.0-p)/n + z*z/(4.0*n*n)).max(0.0)).sqrt();
    ( (center-half).clamp(0.0,1.0), (center+half).clamp(0.0,1.0) )
}
