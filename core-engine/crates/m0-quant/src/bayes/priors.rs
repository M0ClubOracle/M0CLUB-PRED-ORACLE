
#[derive(Debug, Clone, Copy)]
pub struct BetaPrior {
    pub alpha: f64,
    pub beta: f64,
}

impl Default for BetaPrior {
    fn default() -> Self {
        Self { alpha: 1.0, beta: 1.0 }
    }
}
