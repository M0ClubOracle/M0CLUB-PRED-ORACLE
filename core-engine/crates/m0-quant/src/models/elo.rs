
#[derive(Debug, Clone, Copy)]
pub struct EloRating {
    pub r: f64,
}

impl Default for EloRating {
    fn default() -> Self {
        Self { r: 1500.0 }
    }
}

pub fn win_prob(a: EloRating, b: EloRating) -> f64 {
    1.0 / (1.0 + 10f64.powf((b.r - a.r) / 400.0))
}
