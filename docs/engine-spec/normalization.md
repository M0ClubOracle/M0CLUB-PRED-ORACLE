
# Normalization Specification (M0-CORE)

This document specifies how M0Club normalizes heterogeneous inputs into deterministic, versioned internal representations used by the engine pipeline.

Normalization is the process that:
- maps raw source data into canonical types
- enforces schema validation and stable identifiers
- produces deterministic binary payloads for hashing and replay
- assigns quality flags, confidence metadata, and source attribution

This spec is implementation-oriented and should be implemented primarily in `core-engine/m0-ingestor` and shared types in `core-engine/m0-common`.

---

## 1. Goals

- Convert raw data from many domains into a unified `NormalizedEvent` envelope.
- Ensure deterministic representations for hashing and idempotency.
- Provide strict schema versioning for forward compatibility.
- Preserve enough metadata for audits without leaking secrets.
- Support local replay and test vector generation.

Non-goals:
- Performing aggregation or modeling.
- Defining every proprietary data transformation step.

---

## 2. Normalization Pipeline

### 2.1 Steps
1) Parse raw source payload.
2) Validate required fields and basic ranges.
3) Canonicalize identifiers (market_id, outcome ids, teams, tickers).
4) Canonicalize timestamps into UNIX ms.
5) Canonicalize numeric values into fixed-point representations.
6) Construct typed payload bytes using deterministic encoding.
7) Derive stable event_id from canonical fields.
8) Assign quality flags and confidence scores.
9) Emit `NormalizedEvent` to event log.

### 2.2 Determinism constraints
Normalization MUST:
- never depend on locale/timezone formatting
- never depend on floating-point serialization
- never include non-deterministic ordering in payload encoding
- produce stable bytes across platforms for the same input

---

## 3. Canonical Identifiers

### 3.1 MarketId canonicalization
Rules:
- trim whitespace
- uppercase ASCII
- only `A-Z`, `0-9`, `_`
- max length 64 bytes
- reject non-ASCII

If a raw source uses freeform naming:
- map via registry mapping table
- unknown maps go to quarantine

### 3.2 OutcomeId canonicalization
Rules:
- uppercase ASCII
- only `A-Z`, `0-9`, `_`
- max length 64 bytes
- stable across schema versions

### 3.3 SourceId canonicalization
Rules:
- lowercase ASCII recommended for source ids
- max length 64 bytes
Examples:
- `odds_provider_a`
- `solana_logs`
- `macro_calendar`

### 3.4 Entity canonicalization
Sports team codes:
- uppercase short codes (e.g., `LAL`, `BOS`)
Leagues:
- uppercase (e.g., `NBA`, `EPL`)
Political regions:
- uppercase ISO-like codes where possible (e.g., `US`, `UK`)

Mapping tables should be versioned and stored in registry/services.

---

## 4. Timestamp Canonicalization

All timestamps MUST be UNIX epoch milliseconds (UTC).

### 4.1 Parsing rules
If a source provides:
- UNIX seconds: multiply by 1000
- ISO 8601: parse as UTC (or parse with explicit timezone)
- local time: reject unless timezone provided by the source contract

### 4.2 Clock skew checks
Apply checks:
- future skew: allow small skew (e.g., 5 seconds)
- staleness: drop or flag extremely stale timestamps

Set quality flags:
- FUTURE_EVENT_TIME
- STALE_EVENT_TIME

---

## 5. Numeric Canonicalization (Fixed-Point)

### 5.1 Why fixed-point
Floating point is non-deterministic across platforms and must not be used for hashed payload bytes.

### 5.2 Scaling constants
Define scaling constants for numeric domains:

- Probability scale: `P = 1_000_000_000` (1e9)
- Odds scale: `O = 1_000_000` (1e6)
- Price scale: `PX = 1_000_000` (1e6) or `1_000_000_000` depending on required precision
- Rate/percentage scale: `R = 1_000_000`

All scaling constants MUST be treated as protocol constants and versioned if changed.

### 5.3 Rounding rules
When converting to fixed-point:
- compute scaled float
- apply deterministic rounding
- recommended: round to nearest, ties to even

Any component that recomputes these values MUST use identical rounding rules.

---

## 6. Payload Types and Canonical Binary Layouts

Normalization outputs typed payload bytes.
Each payload type has:
- a versioned binary layout
- strict validation rules
- stable canonical ordering for vector fields

### 6.1 Payload type registry
Define a registry of payload types, for example:
- `ODDS_SNAPSHOT_V1`
- `ODDS_TICK_V1`
- `ELECTION_POLL_V1`
- `MACRO_RELEASE_V1`
- `ONCHAIN_PRICE_TICK_V1`
- `ONCHAIN_ACTIVITY_V1`

Each payload includes a `payload_schema_version` in its layout or implied by type enum.

### 6.2 Canonical encoding rules
For all payloads:
- integers are little-endian
- strings are ASCII unless specified, stored as length-prefixed bytes
- arrays are length-prefixed and entries are sorted if order is not semantically meaningful
- optional fields include explicit presence bits

### 6.3 Example layout: ODDS_SNAPSHOT_V1
Fields (logical):
- league
- home_team
- away_team
- market_variant (e.g., moneyline, spread)
- odds_home, odds_away, odds_draw (fixed-point)
- implied probabilities (optional derived)
- snapshot_time_ms

Encoding rules:
- all ids as uppercase ASCII
- store odds as scaled u32 or u64 depending on max
- include a `provider_id` reference as source_id outside payload

### 6.4 Example layout: ONCHAIN_PRICE_TICK_V1
Fields:
- mint/base asset id
- quote asset id
- price_scaled (fixed-point)
- volume_scaled (fixed-point)
- tx_count (u32)
- tick_time_ms

---

## 7. Deriving event_id

`event_id` is a 32-byte sha256 hash.

Recommended derivation:
`event_id = sha256(source_id || market_id || event_time_ms_le || payload_type || payload_bytes)`

Requirements:
- payload_bytes MUST be canonical
- source_id and market_id MUST be canonicalized
- event_time_ms MUST be the canonical event time (not ingest time)

If a source provides a native unique id:
- include it as `source_seq` outside payload
- optionally include it in event_id derivation if stable

---

## 8. Validation Rules

Normalization must enforce strict validation.

### 8.1 Field validation
- required fields present
- lengths within bounds
- ASCII checks
- numeric ranges valid for scaled domain

### 8.2 Consistency validation
- odds must be positive
- probabilities within 0..1 scaled domain
- sums and invariants checked where applicable

### 8.3 Mapping validation
- raw identifiers map to registry-defined MarketId and outcome schema
- unknown identifiers route to quarantine

---

## 9. Quality Flags and Confidence

### 9.1 Input-level confidence
Normalization can assign an input-level confidence score (0..10000):
- based on source quality
- freshness
- completeness
- anomaly checks

This confidence is not the model confidence; it is a raw signal score.

### 9.2 Quality flags
Suggested flags:
- INVALID_SCHEMA
- STALE_EVENT_TIME
- FUTURE_EVENT_TIME
- OUTLIER_DETECTED
- SOURCE_DIVERGENCE
- PARTIAL_COVERAGE
- CONNECTOR_DEGRADED
- QUARANTINED

Flags should be set conservatively and propagated downstream.

---

## 10. Quarantine and Rejection

If normalization fails:
- do not emit to main event log
- emit to quarantine log with reason codes
- store minimal raw metadata for debugging (avoid secrets)

Quarantine record should include:
- source_id
- connector_id
- raw event timestamp
- reason code
- partial parsed fields if safe

---

## 11. Test Vectors

To ensure deterministic compatibility across implementations:
- produce test fixtures containing raw input examples
- produce expected canonical payload bytes (hex)
- produce expected event_id

Store vectors under:
- `core-engine/m0-common/test-vectors/`
- `sdk/*/test-vectors/`

CI must validate:
- recomputed payload bytes match
- event_id matches expected

---

## 12. Implementation Guidance

- Implement canonicalization utilities in `m0-common`:
  - ascii validation
  - uppercasing rules
  - fixed-point conversion with deterministic rounding
  - canonical string encoding helpers
- Enforce maximum lengths to avoid DoS vectors.
- Use feature flags for optional payload types.
- Keep payload layouts stable; version them when changes are required.

---

## Links

- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx
