
# Oracle Output Format (Protocol Spec)

This document specifies the canonical oracle output format produced by M0Club (M0-CORE) and published/verifiable through the on-chain programs.

Goals:
- Deterministic serialization and hashing
- Cross-SDK compatibility
- Verifiable integrity (hash + signatures)
- Extensibility via schema versioning
- Efficient on-chain commitments (commit-reveal) and optional Merkle proofs

This spec defines:
- Logical fields and semantics
- Canonical encoding rules
- Hashing rules
- Signature rules
- Example payloads and test vectors guidance

---

## 1. Terminology

- **Market**: A prediction target with a defined outcome set.
- **Outcome**: A discrete option within a market (e.g., team A win, team B win).
- **Epoch**: A time window for which the oracle output is produced and finalized.
- **Bundle**: A deterministic payload containing oracle outputs for one or more markets/epochs.
- **Bundle Hash**: A stable hash over canonical bundle bytes.
- **Signer Set**: Authorized public keys allowed to sign bundle hashes.
- **Commit-Reveal**: On-chain flow where commit publishes only `bundle_hash`, then reveal publishes bundle bytes or proofs.

---

## 2. Versioning

### 2.1 Schema version
All oracle bundles MUST include:
- `schema_version` (u16 or string)
- Changes to the bundle layout, canonicalization rules, or semantics require a version bump.

Recommended:
- `schema_version = 1` for initial format
- Use semantic changes with explicit migration notes

### 2.2 Backward compatibility
Consumers SHOULD:
- Reject unknown major schema versions
- Gracefully ignore unknown optional fields (when supported)
- Log schema mismatches for operational visibility

---

## 3. Logical Data Model

This section defines the logical fields. Implementations MAY use different in-memory representations, but MUST canonicalize identically for hashing and signing.

### 3.1 OracleBundle (logical)

Required fields:

- `schema_version`
- `bundle_id`
- `produced_at_ms`
- `items[]`
- `integrity`

Optional fields (schema extension pattern):
- `meta` (arbitrary key-value map)
- `tags[]`

#### Field definitions

**schema_version**
- Type: u16 (or string convertible to u16)
- Must be present

**bundle_id**
- Type: 32 bytes (hex string or byte array)
- Stable identifier for the bundle (see Section 5)

**produced_at_ms**
- Type: u64
- UNIX epoch milliseconds when bundle was produced

**items[]**
- Array of `OracleItem` entries
- MUST be stable-sorted by `(market_id, epoch_id, outcome_id)` rules (see Section 4)

**integrity**
- Object containing `bundle_hash`, `signatures`, and replay protection metadata

### 3.2 OracleItem (logical)

Required fields:

- `market_id`
- `epoch_id`
- `window_start_ms`
- `window_end_ms`
- `outcome_id`
- `p`
- `ci_low`
- `ci_high`
- `risk_score`
- `quality_flags`
- `model_id`
- `model_version`

Optional:
- `explanations[]` (references, not raw proprietary signals)
- `features_hash` (commitment to feature snapshot, if enabled)

#### Field definitions

**market_id**
- Type: string
- Canonical: uppercase ASCII, max 64 chars
- Example: `SPORTS_NBA_LAL_BOS_2026_01_04`

**epoch_id**
- Type: u64 or string
- If string, must be canonical ASCII and stable across producers

**window_start_ms / window_end_ms**
- Type: u64
- Inclusive start / exclusive end recommendation

**outcome_id**
- Type: string
- Canonical: uppercase ASCII, max 64 chars
- Example: `HOME_WIN`, `AWAY_WIN`, `DRAW`

**p**
- Type: probability
- Canonical encoding uses fixed precision integer scaling (see Section 4)

**ci_low / ci_high**
- Type: probability
- Same encoding rules as `p`
- Must satisfy `0 <= ci_low <= p <= ci_high <= 1`

**risk_score**
- Type: u16 (0..10000 recommended)
- Interpretation: higher means higher risk/uncertainty

**quality_flags**
- Type: u32 bitmask
- Reserved bits for:
  - source divergence detected
  - stale data
  - low coverage
  - suspected manipulation
  - model degraded mode

**model_id**
- Type: string (ASCII)
- Identifies modeling pipeline family

**model_version**
- Type: string (ASCII)
- Identifies exact model artifact version

---

## 4. Canonical Encoding Rules

Canonical encoding MUST be identical across implementations.

### 4.1 Encoding choice
Two canonical encodings are defined:

- **Binary canonical encoding (recommended for hashing and on-chain reveal)**
  - Borsh-like strict layout OR custom fixed layout
  - Deterministic ordering and fixed integer scaling
- **Canonical JSON (human readable)**
  - Only for external display and debugging
  - MUST NOT be used for hashing unless canonicalized strictly

This spec defines the binary canonical encoding.

### 4.2 Normalization rules
- All identifiers MUST be ASCII and uppercased where specified.
- Whitespace MUST be trimmed from identifiers.
- Reject non-ASCII identifiers in canonical encoding.

### 4.3 Sorting rules for items
Items MUST be sorted by:
1) `market_id` (lexicographic byte order)
2) `epoch_id` (if numeric: ascending; if string: lexicographic)
3) `outcome_id` (lexicographic)
4) `window_start_ms` (ascending)
5) `window_end_ms` (ascending)

### 4.4 Probability integer scaling
To avoid float nondeterminism:
- Encode probabilities as `u32` fixed-point integers.
- Define scale `S = 1_000_000_000` (1e9) by default.

Conversion:
- `p_scaled = round(p * S)` with ties to nearest even recommended
- Range: 0..S
- Same for `ci_low_scaled`, `ci_high_scaled`

Validation:
- `p_scaled <= S`
- `ci_low_scaled <= p_scaled <= ci_high_scaled`
- All are within 0..S

### 4.5 Binary layout (schema_version = 1)

All integers are little-endian.

#### OracleBundleV1
- `schema_version: u16`
- `bundle_id: [u8; 32]`
- `produced_at_ms: u64`
- `item_count: u32`
- `items: OracleItemV1[item_count]`
- `integrity: IntegrityV1`

#### OracleItemV1
- `market_id_len: u16`
- `market_id_bytes: [u8; market_id_len]`
- `epoch_id_kind: u8` (0 = u64, 1 = string)
- `epoch_id_u64: u64` (if kind=0 else 0)
- `epoch_id_len: u16` (if kind=1 else 0)
- `epoch_id_bytes: [u8; epoch_id_len]` (if kind=1)
- `window_start_ms: u64`
- `window_end_ms: u64`
- `outcome_id_len: u16`
- `outcome_id_bytes: [u8; outcome_id_len]`
- `p_scaled: u32`
- `ci_low_scaled: u32`
- `ci_high_scaled: u32`
- `risk_score: u16`
- `quality_flags: u32`
- `model_id_len: u16`
- `model_id_bytes: [u8; model_id_len]`
- `model_version_len: u16`
- `model_version_bytes: [u8; model_version_len]`
- `explanations_count: u16`
- `explanations: ExplanationRefV1[explanations_count]`
- `features_hash_present: u8` (0/1)
- `features_hash: [u8; 32]` (if present else all zeros)

#### ExplanationRefV1
- `kind: u8` (0=url, 1=doc, 2=note)
- `ref_len: u16`
- `ref_bytes: [u8; ref_len]`

#### IntegrityV1
- `bundle_hash: [u8; 32]`
- `signer_set_id: u32`
- `sequence: u64`
- `signature_count: u16`
- `signatures: SignatureV1[signature_count]`

#### SignatureV1
- `pubkey: [u8; 32]`
- `sig: [u8; 64]` (ed25519)
- `sig_scheme: u8` (0 = ed25519)
- `flags: u16` (reserved)

---

## 5. Bundle ID and Hashing

### 5.1 bundle_id
`bundle_id` SHOULD be a 32-byte identifier derived from stable fields, for example:

`bundle_id = sha256(schema_version || produced_at_ms || first_item.market_id || first_item.epoch_id)`

The exact derivation can be implementation-defined, but MUST be stable and deterministic for the same bundle.

### 5.2 bundle_hash
`bundle_hash` MUST be:

`bundle_hash = sha256(canonical_bundle_bytes)`

Where `canonical_bundle_bytes` is the exact bytes of `OracleBundleV1` with `integrity.bundle_hash` temporarily zeroed during hashing to avoid self-reference.

Hashing procedure:
1) Create `OracleBundleV1` with `integrity.bundle_hash = 0x00..00` (32 bytes)
2) Serialize canonical bytes
3) Compute SHA-256
4) Set `integrity.bundle_hash` to computed hash

---

## 6. Signing

### 6.1 Signature payload
Signers MUST sign:
- `bundle_hash` (32 bytes)
- plus replay protection context:
  - `signer_set_id` (u32 LE)
  - `sequence` (u64 LE)

Recommended signature message:
`msg = sha256("M0CLUB_ORACLE_V1" || bundle_hash || signer_set_id || sequence)`

Where `"M0CLUB_ORACLE_V1"` is ASCII.

### 6.2 Signature scheme
- Default: ed25519
- `sig_scheme = 0` indicates ed25519

### 6.3 Verification requirements
Consumers MUST verify:
- signature belongs to an authorized signer in signer set
- signature message matches the bundle hash and replay context
- sequence is not reused for the same signer set (enforced off-chain and on-chain as applicable)

---

## 7. Merkle Proof Mode (Optional)

For large bundles, the reveal transaction may not include the full payload. In that case:
- Commit publishes the Merkle root
- Reveal provides a proof path for a specific item (or chunk)

Recommended approach:
- Compute Merkle tree over canonical `OracleItemV1` bytes
- Root becomes `bundle_hash` or `merkle_root` (choose one and keep consistent in on-chain logic)
- Consumers verify membership proofs

This repository may implement:
- Full bundle reveal for localnet/smaller updates
- Merkle proof reveal for mainnet constraints

---

## 8. Canonical JSON (Debug Only)

Canonical JSON is for humans and external systems. It MUST NOT be used for hashing unless strict canonicalization is implemented.

Recommended JSON fields:
- `schema_version`
- `bundle_id`
- `produced_at_ms`
- `items[]`
- `integrity`

Probability values should be represented both as:
- float (for display)
- `*_scaled` integers (for verification)

---

## 9. Example Payload (Debug JSON)

```json
{
  "schema_version": 1,
  "bundle_id": "1f2e3d...",
  "produced_at_ms": 1767532800000,
  "items": [
    {
      "market_id": "SPORTS_NBA_LAL_BOS_2026_01_04",
      "epoch_id": 42,
      "window_start_ms": 1767532800000,
      "window_end_ms": 1767536400000,
      "outcome_id": "HOME_WIN",
      "p_scaled": 531000000,
      "ci_low_scaled": 490000000,
      "ci_high_scaled": 570000000,
      "risk_score": 1200,
      "quality_flags": 0,
      "model_id": "M0CORE_ENS_V1",
      "model_version": "2026-01-04.1",
      "explanations": [
        { "kind": 0, "ref": "https://m0club.com/" }
      ],
      "features_hash": null
    }
  ],
  "integrity": {
    "bundle_hash": "aa11bb...",
    "signer_set_id": 1,
    "sequence": 999,
    "signatures": [
      {
        "pubkey": "....",
        "sig": "....",
        "sig_scheme": 0,
        "flags": 0
      }
    ]
  }
}
```

---

## 10. Test Vectors Guidance

To ensure compatibility across SDKs:
- Provide a known input bundle in JSON
- Provide the canonical bytes as a hex string
- Provide the expected `bundle_hash`
- Provide expected signature message hash (the `msg` preimage hash)
- Provide a valid signature from a test key

Place vectors under:
- `sdk/ts/test-vectors/`
- `sdk/rust/tests/vectors/`
- `sdk/python/tests/vectors/`

CI should validate:
- bytes match
- hash matches
- signatures verify

---

## Links

- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx
