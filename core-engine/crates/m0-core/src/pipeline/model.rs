
use m0_quant::models::elo::{EloRating, win_prob};
use m0_quant::ProbabilityPoint;
use m0_quant::confidence::ci::wilson_ci;

pub fn predict_two_outcome(outcome_a: &str, outcome_b: &str, rating_a: f64, rating_b: f64, samples: u64) -> Vec<ProbabilityPoint> {
    let p = win_prob(EloRating { r: rating_a }, EloRating { r: rating_b });

    // Approximate CI using Wilson with n=samples.
    let (lo, hi) = wilson_ci(p, samples as f64, 1.96);
    vec![
        ProbabilityPoint { outcome_id: outcome_a.to_string(), p, ci_low: lo, ci_high: hi, ci_level: 0.95, quality_flags: 0 },
        ProbabilityPoint { outcome_id: outcome_b.to_string(), p: 1.0-p, ci_low: 1.0-hi, ci_high: 1.0-lo, ci_level: 0.95, quality_flags: 0 },
    ]
}
