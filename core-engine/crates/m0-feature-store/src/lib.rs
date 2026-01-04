
pub mod error;
pub mod schema;
pub mod storage;
pub mod transforms;

use m0_normalizer::schema::canonical::CanonicalEvent;
use schema::feature_proto::FeatureRow;

pub fn to_feature_row(ev: &CanonicalEvent) -> FeatureRow {
    FeatureRow {
        market_id: ev.market_id.clone(),
        ts_ms: ev.observed_at_ms,
        features: ev.features.clone(),
    }
}
