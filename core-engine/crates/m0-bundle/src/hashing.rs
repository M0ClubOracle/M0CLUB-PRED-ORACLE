
use sha2::{Digest, Sha256};

pub fn bundle_content_hash(json_bytes: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"M0_BUNDLE_CONTENT_V1");
    h.update(json_bytes);
    h.finalize().into()
}
