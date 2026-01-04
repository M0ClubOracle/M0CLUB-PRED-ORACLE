
use crate::error::FeatureStoreError;
use crate::schema::feature_proto::FeatureRow;

#[derive(Debug, Clone)]
pub struct PostgresStore {
    pub dsn: String,
}

impl PostgresStore {
    pub fn new(dsn: impl Into<String>) -> Self {
        Self { dsn: dsn.into() }
    }

    pub async fn write(&self, _row: &FeatureRow) -> Result<(), FeatureStoreError> {
        // Skeleton: implement real DB writes later.
        Ok(())
    }
}
