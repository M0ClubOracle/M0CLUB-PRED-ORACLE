
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BundleId(pub Uuid);

impl BundleId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_bytes16(&self) -> [u8; 16] {
        *self.0.as_bytes()
    }
}
