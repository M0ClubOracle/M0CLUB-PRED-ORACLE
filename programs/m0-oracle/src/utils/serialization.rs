
use anchor_lang::prelude::*;

pub fn pack_u64_le(x: u64) -> [u8; 8] {
    x.to_le_bytes()
}

pub fn pack_u32_le(x: u32) -> [u8; 4] {
    x.to_le_bytes()
}

pub fn pack_u16_le(x: u16) -> [u8; 2] {
    x.to_le_bytes()
}
