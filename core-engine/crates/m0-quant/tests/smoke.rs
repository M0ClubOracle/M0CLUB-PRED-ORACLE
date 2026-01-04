
use m0_quant::models::elo::{win_prob, EloRating};

#[test]
fn elo_prob_range() {
    let p = win_prob(EloRating { r: 1500.0 }, EloRating { r: 1600.0 });
    assert!(p >= 0.0 && p <= 1.0);
}
