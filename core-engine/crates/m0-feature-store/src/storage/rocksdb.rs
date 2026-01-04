
use crate::error::FeatureStoreError;
use crate::schema::feature_proto::FeatureRow;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RocksStore {
    pub path: PathBuf,
}

impl RocksStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub async fn write(&self, _row: &FeatureRow) -> Result<(), FeatureStoreError> {
        // Skeleton: implement rocksdb later.
        Ok(())
    }
}
