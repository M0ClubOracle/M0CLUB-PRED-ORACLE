
use m0_common::hashing::sha256_hex;

#[test]
fn hash_is_stable() {
    assert_eq!(sha256_hex(b"m0"), sha256_hex(b"m0"));
}
