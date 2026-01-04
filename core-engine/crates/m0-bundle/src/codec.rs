
use crate::format::Bundle;

pub fn encode_json(bundle: &Bundle) -> anyhow::Result<Vec<u8>> {
    Ok(serde_json::to_vec(bundle)?)
}

pub fn decode_json(bytes: &[u8]) -> anyhow::Result<Bundle> {
    Ok(serde_json::from_slice(bytes)?)
}
