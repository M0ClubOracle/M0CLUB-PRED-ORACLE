
use sha2::{Digest, Sha256};

pub fn commit_hash(bundle_content_hash: &[u8; 32], salt: &[u8; 32]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"M0_COMMIT_V1");
    h.update(bundle_content_hash);
    h.update(salt);
    h.finalize().into()
}
