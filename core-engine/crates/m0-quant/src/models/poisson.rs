
pub fn poisson_pmf(k: u32, lambda: f64) -> f64 {
    if lambda <= 0.0 { return 0.0; }
    let mut fact = 1.0;
    for i in 1..=k { fact *= i as f64; }
    (lambda.powi(k as i32) * (-lambda).exp()) / fact
}
