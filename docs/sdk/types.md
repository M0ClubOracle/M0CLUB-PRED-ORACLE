
# SDK Types

This document defines the canonical SDK data types for M0Club.
These types are shared across:
- TypeScript SDK
- Rust SDK
- Python SDK (as JSON schemas / dataclasses)

All SDKs must preserve:
- field names
- canonical ordering rules where required (hashing)
- integer scaling and ranges
- schema_versioning semantics

This doc is intentionally explicit so SDKs can be implemented consistently.

Links:
- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx

---

## 0. Conventions

### 0.1 JSON vs canonical bytes
- SDKs exchange JSON over HTTP/WebSocket for developer convenience.
- Canonical bytes used for hashing and signing are defined in `docs/engine-spec/bundle-hashing.md`.
- The JSON types below must map 1:1 to canonical fields.

### 0.2 Scaling
Probabilities and ratios are fixed-point integers:
- Probability scale: `P = 1_000_000_000` (1e9)
- Risk score scale: `R = 10_000` (basis points-like, 0..10000)
- Confidence level scale: bps (0..10000)

All numbers must be integers in transport.
No floats are permitted in protocol payloads.

### 0.3 Identifiers
- `market_id`, `outcome_id`, `bundle_id` are ASCII strings with strict max length (recommended <= 64).
- Canonical ordering is ASCII ascending by bytes.

### 0.4 Time
Times are milliseconds since Unix epoch:
- `created_at_ms`
- `observed_at_ms`
- `updated_at_ms`

### 0.5 Versioning
- `schema_version` is required in bundles.
- SDK must reject unknown major schema versions unless explicitly configured.

---

## 1. Core Types

### 1.1 MarketId
```ts
type MarketId = string; // ASCII, <= 64 chars
```

### 1.2 OutcomeId
```ts
type OutcomeId = string; // ASCII, <= 64 chars
```

### 1.3 EpochId
```ts
type EpochId = number; // u64
```

### 1.4 TickIndex
```ts
type TickIndex = number; // u32
```

---

## 2. Oracle Output Types

### 2.1 OutcomeProbability
Represents one outcome within a market.

Fields:
- `outcome_id`: OutcomeId
- `p_scaled`: u32/u64 (0..P)
- `ci_low_scaled`: u32/u64 (0..P)
- `ci_high_scaled`: u32/u64 (0..P)
- `ci_level_bps`: u16 (e.g., 9000, 9500, 9900)
- `quality_flags`: u32 bitmask (optional per outcome)
- `aux`: optional map for non-hashed metadata (must not affect bundle hashing)

TypeScript interface:
```ts
export interface OutcomeProbability {
  outcome_id: string;
  p_scaled: string;        // use string for u64 safety
  ci_low_scaled: string;
  ci_high_scaled: string;
  ci_level_bps: number;
  quality_flags?: number;
  aux?: Record<string, unknown>;
}
```

Rust struct:
```rust
pub struct OutcomeProbability {
    pub outcome_id: String,
    pub p_scaled: u64,
    pub ci_low_scaled: u64,
    pub ci_high_scaled: u64,
    pub ci_level_bps: u16,
    pub quality_flags: u32,
}
```

Python dataclass:
```py
from dataclasses import dataclass
from typing import Optional, Dict, Any

@dataclass
class OutcomeProbability:
    outcome_id: str
    p_scaled: int
    ci_low_scaled: int
    ci_high_scaled: int
    ci_level_bps: int
    quality_flags: int = 0
    aux: Optional[Dict[str, Any]] = None
```

Notes:
- TypeScript uses strings for u64 fields to avoid precision loss.
- `aux` must never be included in canonical hashing.

---

### 2.2 MarketOutput
Represents the output for one market.

Fields:
- `market_id`: MarketId
- `epoch_id`: u64
- `tick_index`: u32
- `sequence`: u64 (replay protection)
- `observed_at_ms`: u64
- `model_version`: string (<= 32, informational; may be included in hashing only if specified)
- `risk_score`: u16 (0..R)
- `quality_flags`: u32 bitmask
- `outcomes`: OutcomeProbability[] (sorted by outcome_id ASCII)
- `evidence_hash`: 32-byte hex string (optional)
- `features_hash`: 32-byte hex string (optional)
- `guardrail_reason_codes`: string[] (optional; informational unless included in hashing)
- `meta`: optional non-hashed metadata

TypeScript:
```ts
export interface MarketOutput {
  market_id: string;
  epoch_id: string;       // u64 as string
  tick_index: number;     // u32
  sequence: string;       // u64 as string
  observed_at_ms: string; // u64 as string
  model_version: string;
  risk_score: number;     // u16
  quality_flags: number;  // u32
  outcomes: OutcomeProbability[];
  evidence_hash?: string;
  features_hash?: string;
  guardrail_reason_codes?: string[];
  meta?: Record<string, unknown>;
}
```

Notes:
- `outcomes` must be sorted by `outcome_id` ASCII.
- `meta` and optional lists must not affect canonical hashing unless explicitly specified by schema version.

---

### 2.3 OracleBundle
A bundle contains many market outputs plus signatures.

Fields:
- `schema_version`: u16
- `bundle_id`: string (unique id, recommended UUIDv7 or hash-derived)
- `created_at_ms`: u64
- `publish_epoch_id`: u64
- `commit_reveal_mode`: string enum (`single_tx`, `merkle_chunks`)
- `signer_set_id`: u64
- `sequence_base`: u64 (optional, if sequences are derived)
- `markets`: MarketOutput[] (sorted by market_id ASCII)
- `bundle_content_hash`: 32-byte hex string
- `signatures`: BundleSignature[] (sorted by signer_pubkey bytes)

TypeScript:
```ts
export interface OracleBundle {
  schema_version: number;
  bundle_id: string;
  created_at_ms: string;
  publish_epoch_id: string;
  commit_reveal_mode: "single_tx" | "merkle_chunks";
  signer_set_id: string;
  sequence_base?: string;
  markets: MarketOutput[];
  bundle_content_hash: string; // hex
  signatures: BundleSignature[];
}
```

Notes:
- `bundle_content_hash` is computed from canonical content bytes without signatures.
- `markets` must be sorted by market_id ASCII.
- `signatures` must be sorted by signer_pubkey bytes.

---

### 2.4 BundleSignature
Represents one signature over the signature message hash.

Fields:
- `signer_pubkey`: base58 or hex (protocol-defined; recommend base58 for Solana keys)
- `signature`: base58 or hex signature bytes
- `algo`: string enum (`ed25519`)
- `created_at_ms`: u64 (optional, informational)

TypeScript:
```ts
export interface BundleSignature {
  signer_pubkey: string;
  signature: string;
  algo: "ed25519";
  created_at_ms?: string;
}
```

---

## 3. Quality Flags

Quality flags represent conditions that affect trust/risk.
Flags are u32 bitmasks.

Recommended bit assignments (v1):
- 0: LOW_COVERAGE
- 1: HIGH_DIVERGENCE
- 2: STALE_INPUTS
- 3: OUTLIER_SPIKE
- 4: JUMP_EXCEEDED
- 5: DRIFT_DETECTED
- 6: SIGNER_UNAVAILABLE
- 7: RPC_UNHEALTHY
- 8: CI_INVALID
- 9: CI_NOT_AVAILABLE

SDKs should expose helpers:
- `hasFlag(flags, Flag.LOW_COVERAGE)`
- `formatFlags(flags)`

---

## 4. Registry Types

### 4.1 MarketDefinition
Defines a market and its configuration.

Fields:
- `market_id`
- `domain`: string enum (`sports`, `politics`, `macro`, `onchain`, `custom`)
- `tier_policy`: string enum (`FAST`, `NORMAL`, `STRICT`)
- `cadence_ms`: u32
- `outcomes`: string[] (outcome ids)
- `active`: boolean
- `created_at_ms`: u64
- `updated_at_ms`: u64

TypeScript:
```ts
export interface MarketDefinition {
  market_id: string;
  domain: string;
  tier_policy: "FAST" | "NORMAL" | "STRICT";
  cadence_ms: number;
  outcomes: string[];
  active: boolean;
  created_at_ms: string;
  updated_at_ms: string;
}
```

### 4.2 SignerSet
Defines the signer set.

Fields:
- `signer_set_id`: u64
- `threshold`: u16
- `pubkeys`: string[]
- `active`: boolean
- `activation_epoch_id`: u64 (optional)
- `created_at_ms`: u64
- `meta`: optional

TypeScript:
```ts
export interface SignerSet {
  signer_set_id: string;
  threshold: number;
  pubkeys: string[];
  active: boolean;
  activation_epoch_id?: string;
  created_at_ms: string;
  meta?: Record<string, unknown>;
}
```

---

## 5. API Response Types (Convenience)

### 5.1 MarketLatestResponse
Represents the common API payload for the latest market output.

Fields:
- `market_id`
- `epoch_id`
- `tick_index`
- `bundle_content_hash`
- `signer_set_id`
- `bundle`: OracleBundle
- `verified`: boolean (optional; server-side verification)

TypeScript:
```ts
export interface MarketLatestResponse {
  market_id: string;
  epoch_id: string;
  tick_index: number;
  bundle_content_hash: string;
  signer_set_id: string;
  bundle: OracleBundle;
  verified?: boolean;
}
```

---

## 6. Canonical Ordering Rules (SDK Requirements)

All SDKs MUST:
- sort markets by `market_id` ASCII before hashing/verifying
- sort outcomes by `outcome_id` ASCII before hashing/verifying
- sort signatures by signer_pubkey bytes before verifying threshold logic

When parsing JSON from untrusted sources:
- do not assume ordering is correct
- reorder in-memory before hashing/verifying

---

## 7. Validation Rules

SDKs should implement validation utilities.

### 7.1 Probability validation
- 0 <= p_scaled <= P
- sum(outcome p_scaled) == P (after canonical normalization)
- ci_low <= p <= ci_high

### 7.2 Bundle validation
- schema_version supported
- bundle_content_hash matches computed hash
- signatures verify and threshold met

### 7.3 Registry validation
- market definitions include unique outcome ids
- signer set pubkeys unique
- threshold <= pubkeys length

---

## 8. JSON Schemas (Recommended)

For interoperability, provide JSON schemas:
- `schemas/oracle-bundle.v1.json`
- `schemas/market-definition.v1.json`
- `schemas/signer-set.v1.json`

SDKs can generate types from these schemas.

---

## 9. Compatibility Notes

- TypeScript must use strings for u64 fields to avoid precision issues.
- Python int supports big integers; still validate bounds.
- Rust uses u64 and requires careful JSON parsing (serde with string support).

---

## 10. References

- Bundle hashing: `docs/engine-spec/bundle-hashing.md`
- Output format: `docs/protocol-spec/oracle-output-format.md`
- Commit-reveal: `docs/protocol-spec/commit-reveal.md`
- Replay protection: `docs/protocol-spec/replay-protection.md`
- Signer set: `docs/protocol-spec/signer-set.md`

---

## Links

- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx
