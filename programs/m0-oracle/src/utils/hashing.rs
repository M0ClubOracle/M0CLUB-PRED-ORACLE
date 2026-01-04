
use anchor_lang::prelude::*;
use sha2::{Digest, Sha256};

// Domain-separated hashing helpers.
// These are intentionally simple; the canonical format is documented in docs/engine-spec/bundle-hashing.md.
// For production, keep the format stable and versioned.

pub fn hash_commit(bundle_hash: &[u8; 32], salt: &[u8; 32]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"M0_COMMIT_V1");
    h.update(bundle_hash);
    h.update(salt);
    h.finalize().into()
}

pub fn hash_bundle_content(payload: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"M0_BUNDLE_CONTENT_V1");
    h.update(payload);
    h.finalize().into()
}

pub fn hash_signature_message(bundle_content_hash: &[u8; 32], signer_set_id: u64, publish_epoch_id: u64, sequence: u64) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"M0_SIGMSG_V1");
    h.update(bundle_content_hash);
    h.update(signer_set_id.to_le_bytes());
    h.update(publish_epoch_id.to_le_bytes());
    h.update(sequence.to_le_bytes());
    h.finalize().into()
}
