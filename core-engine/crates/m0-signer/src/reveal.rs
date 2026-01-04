
use sha2::{Digest, Sha256};

pub fn signature_message(bundle_content_hash: &[u8; 32], signer_set_id: u64, publish_epoch_id: u64, sequence: u64) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"M0_SIGMSG_V1");
    h.update(bundle_content_hash);
    h.update(signer_set_id.to_le_bytes());
    h.update(publish_epoch_id.to_le_bytes());
    h.update(sequence.to_le_bytes());
    h.finalize().into()
}
