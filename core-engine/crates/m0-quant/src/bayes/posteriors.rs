
use super::priors::BetaPrior;

#[derive(Debug, Clone, Copy)]
pub struct BetaPosterior {
    pub alpha: f64,
    pub beta: f64,
}

impl BetaPosterior {
    pub fn mean(&self) -> f64 {
        self.alpha / (self.alpha + self.beta).max(1e-12)
    }
}

impl From<(BetaPrior, u64, u64)> for BetaPosterior {
    fn from((p, wins, losses): (BetaPrior, u64, u64)) -> Self {
        Self { alpha: p.alpha + wins as f64, beta: p.beta + losses as f64 }
    }
}
