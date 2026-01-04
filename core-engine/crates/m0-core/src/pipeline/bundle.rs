
use m0_bundle::format::{Bundle, MarketReveal, OutcomePoint};
use m0_bundle::codec::encode_json;
use m0_bundle::hashing::bundle_content_hash;
use m0_common::ids::BundleId;
use m0_common::time::now_ms;
use m0_quant::ProbabilityPoint;

pub fn build_bundle(schema_version: u16, signer_set_id: u64, publish_epoch_id: u64, market_id: &str, epoch_id: u64, tick_index: u32, sequence: u64, risk_score: u16, probs: &[ProbabilityPoint]) -> anyhow::Result<(Bundle, Vec<u8>, [u8; 32])> {
    let outcomes: Vec<OutcomePoint> = probs.iter().map(|p| OutcomePoint {
        outcome_id: p.outcome_id.clone(),
        p_scaled: (p.p * 1_000_000_000.0).round() as u64,
        ci_low_scaled: (p.ci_low * 1_000_000_000.0).round() as u64,
        ci_high_scaled: (p.ci_high * 1_000_000_000.0).round() as u64,
        ci_level_bps: (p.ci_level * 10_000.0).round() as u16,
        quality_flags: p.quality_flags,
    }).collect();

    let mr = MarketReveal {
        market_id: market_id.to_string(),
        epoch_id,
        tick_index,
        sequence,
        observed_at_ms: now_ms(),
        risk_score,
        quality_flags: 0,
        outcomes,
    };

    let b = Bundle {
        schema_version,
        signer_set_id,
        publish_epoch_id,
        created_at_ms: now_ms(),
        bundle_id: BundleId::new(),
        markets: vec![mr],
    };

    let bytes = encode_json(&b)?;
    let h = bundle_content_hash(&bytes);
    Ok((b, bytes, h))
}
