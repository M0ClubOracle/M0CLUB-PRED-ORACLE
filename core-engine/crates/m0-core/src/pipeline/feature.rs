
use m0_feature_store::to_feature_row;
use m0_normalizer::schema::canonical::CanonicalEvent;
use m0_feature_store::schema::feature_proto::FeatureRow;

pub fn make_features(ev: &CanonicalEvent) -> FeatureRow {
    to_feature_row(ev)
}
