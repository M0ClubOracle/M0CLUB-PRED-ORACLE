
use super::priors::BetaPrior;
use super::posteriors::BetaPosterior;

pub fn infer_beta(prior: BetaPrior, wins: u64, losses: u64) -> BetaPosterior {
    (prior, wins, losses).into()
}
