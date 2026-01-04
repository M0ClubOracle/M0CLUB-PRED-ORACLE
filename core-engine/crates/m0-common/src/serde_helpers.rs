
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize_hex_32<S>(bytes: &[u8; 32], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&hex::encode(bytes))
}

pub fn deserialize_hex_32<'de, D>(d: D) -> Result<[u8; 32], D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    let raw = hex::decode(&s).map_err(serde::de::Error::custom)?;
    if raw.len() != 32 {
        return Err(serde::de::Error::custom("expected 32 bytes"));
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&raw);
    Ok(out)
}
