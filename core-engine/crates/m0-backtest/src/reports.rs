
use crate::metrics::Metrics;

pub fn render_text(m: &Metrics) -> String {
    format!("brier_mean={:.6}\nlogloss_mean={:.6}\n", m.brier_mean, m.logloss_mean)
}
