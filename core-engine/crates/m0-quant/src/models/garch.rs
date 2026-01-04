
#[derive(Debug, Clone, Copy)]
pub struct Garch11 {
    pub omega: f64,
    pub alpha: f64,
    pub beta: f64,
}

impl Default for Garch11 {
    fn default() -> Self {
        Self { omega: 1e-6, alpha: 0.05, beta: 0.9 }
    }
}

pub fn step(model: Garch11, prev_var: f64, prev_ret: f64) -> f64 {
    model.omega + model.alpha * prev_ret.powi(2) + model.beta * prev_var
}
