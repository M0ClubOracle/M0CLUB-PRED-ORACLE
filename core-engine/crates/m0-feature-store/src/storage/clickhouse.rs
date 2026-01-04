
use crate::error::FeatureStoreError;
use crate::schema::feature_proto::FeatureRow;

#[derive(Debug, Clone)]
pub struct ClickhouseStore {
    pub url: String,
}

impl ClickhouseStore {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    pub async fn write(&self, _row: &FeatureRow) -> Result<(), FeatureStoreError> {
        Ok(())
    }
}
