
# Aggregation and Modeling Windows (Protocol Spec)

This document specifies how M0Club aggregates normalized input signals into modeling windows, how features are computed, and how aggregated results are emitted as oracle outputs.

The goal is to define:
- deterministic aggregation windows aligned to epoch/tick boundaries
- consistent feature computation across deployments
- minimal contracts between ingestion, modeling, and bundling
- operational constraints for low-latency real-time analytics

This spec focuses on aggregation mechanics and does not prescribe specific proprietary model internals. It defines the interfaces that models must satisfy.

---

## 1. Principles

1) Deterministic time alignment
- Every component must agree on window boundaries.
- Use integer milliseconds in UTC.
- Prefer registry-configured epoch and tick boundaries.

2) Idempotent aggregation
- Replaying events produces identical aggregates.
- Aggregates are keyed by stable window identifiers.

3) Multi-source robustness
- Inputs may arrive late, out of order, or duplicated.
- Aggregation must tolerate and report data quality issues.

4) Model-agnostic contracts
- Aggregation produces features; modeling consumes features.
- Feature schema is versioned and audited.

---

## 2. Inputs: Normalized Events

Ingestion connectors must normalize all source events into a common envelope.

### 2.1 NormalizedEvent (logical)

Required fields:
- `event_id` (32 bytes) stable hash
- `source_id` (string)
- `market_id` (string)
- `event_time_ms` (u64) timestamp when the signal occurred
- `ingest_time_ms` (u64) timestamp when it was ingested
- `payload_type` (enum)
- `payload` (typed)
- `quality_flags` (u32 bitmask)
- `schema_version` (u16)

Optional:
- `partition_key` (string) for sharding
- `trace_id` (string) for observability correlation

### 2.2 event_id stability
`event_id` must be stable across re-ingestion of the same raw signal.
Recommended:
`event_id = sha256(source_id || market_id || event_time_ms || canonical_source_payload)`

### 2.3 Quality flags
Quality flags must allow aggregation and modeling to detect issues:
- stale
- missing fields
- divergent sources
- suspected manipulation
- partial coverage
- connector degraded mode

---

## 3. Window Definitions

Aggregation windows are aligned to epoch/tick boundaries configured per market.

Registry parameters:
- `epoch_window_ms`
- `publish_cadence_ms`
- `origin_ms` (protocol constant)

See `epoch-rounding.md` for derivation.

### 3.1 Window keys
Aggregation is keyed by:
- `market_id`
- `epoch_id`
- `tick_index` (optional)
- `window_start_ms`
- `window_end_ms`
- `feature_schema_version`

Recommended window identifier:
`window_key = sha256(market_id || epoch_id || tick_index || window_start_ms || window_end_ms || feature_schema_version)`

### 3.2 Types of windows
M0Club commonly uses:
- **tick window**: aligned to publish cadence within an epoch
- **epoch window**: full epoch range
- **rolling windows**: multiple sizes for features (e.g., 5m, 15m, 1h) aligned to ticks

Rolling windows must be derived deterministically from tick boundaries:
- for a rolling size `R_ms`, define window end at `tick_end_ms` and start at `tick_end_ms - R_ms`

---

## 4. Watermarks and Late Data

### 4.1 Watermark
A watermark is a monotonic value representing the latest processed event_time_ms (per market partition).
Used for:
- detecting lag
- bounding late arrival handling
- controlling backfill/replay

### 4.2 Allowed lateness
Markets can define allowed lateness:
- `allowed_lateness_ms` (u64)

Events with `event_time_ms < (watermark - allowed_lateness_ms)` are considered too late.
Handling options:
- drop and mark a counter
- include in a correction path (future tick) as an adjustment feature

Recommended default:
- drop too-late events and surface a metric

### 4.3 Out-of-order
Aggregation should accept out-of-order events within allowed lateness.
Event-time ordering is used for deterministic state updates.

---

## 5. Aggregation State Machine

Aggregation is performed per (market, window).

### 5.1 State
An AggregationState holds:
- window bounds
- counters and summaries
- distinct source statistics
- data quality metrics
- feature values (intermediate)

### 5.2 Deterministic update rule
For each event:
1) Determine the windows the event belongs to.
2) Apply update functions that are associative/commutative where possible.
3) Ensure stable ordering for non-commutative updates by sorting on `(event_time_ms, event_id)` during replay.

### 5.3 Window membership
An event belongs to a window if:
`window_start_ms <= event_time_ms < window_end_ms`

If multiple window sizes are used, the event may update multiple states.

---

## 6. Feature Schema

Features are the contract between aggregation and modeling.

### 6.1 FeatureSchema
A FeatureSchema is identified by:
- `feature_schema_version` (u16)
- `domain` (enum)
- `market_id` (optional specialization)
- `features[]` definitions

Each feature definition includes:
- `name` (ASCII)
- `dtype` (enum: i64, u64, f64_scaled_u32, bool, string_ref)
- `description`
- `required` (bool)
- `default` (optional)
- `aggregation_rule` (reference string)

### 6.2 Fixed-point features
To preserve determinism:
- Prefer integer scaling for float-like features.
- Use the same scaling constants across implementations.
- Store scale factors in schema metadata.

### 6.3 Example feature groups
Common feature groups:
- volume and count metrics
- source divergence metrics
- latency and freshness metrics
- odds/price deltas
- implied probability changes
- volatility proxies
- sentiment proxies (if included)

---

## 7. Aggregation Outputs

Aggregation produces a FeatureVector per window.

### 7.1 FeatureVector (logical)
Required:
- `market_id`
- `epoch_id`
- `tick_index`
- `window_start_ms`
- `window_end_ms`
- `feature_schema_version`
- `features` (map name -> value)
- `quality_flags` (u32)
- `coverage` (object)
- `features_hash` (32 bytes optional commitment)

### 7.2 coverage object
Coverage captures how complete the inputs are:
- `sources_expected` (u16)
- `sources_seen` (u16)
- `events_seen` (u32)
- `late_events_dropped` (u32)
- `stale_events_seen` (u32)

### 7.3 features_hash
If enabled, compute:
`features_hash = sha256(canonical_feature_bytes)`
This hash can be included in oracle bundles as an integrity reference without revealing raw features.

---

## 8. Modeling Window Consumption

Models consume FeatureVectors.

### 8.1 ModelInput
ModelInput includes:
- FeatureVector
- model config and params
- optional historical context pointer
- backtest calibration context

### 8.2 ModelOutput
Models produce:
- outcome distribution probabilities
- confidence interval metadata
- risk scores
- explanation references (non-sensitive)
- quality flags propagated and augmented

ModelOutput is then bundled per `oracle-output-format.md`.

---

## 9. Determinism Rules

To ensure reproducible outputs:
- All time is UTC unix ms.
- All identifiers are ASCII and canonicalized.
- Sorting of events in replay uses `(event_time_ms, event_id)`.
- All float-like values use fixed-point integer scaling.
- Feature schema versions are explicit.
- Window derivation uses registry parameters.

---

## 10. Operational Constraints

### 10.1 Latency budgets
Typical budgets:
- ingestion normalize: < 100ms (domain dependent)
- aggregation update: < 10ms per event batch
- modeling compute: < 50-500ms depending on model
- bundling + signing: < 50ms
- commit/reveal: network dependent

### 10.2 Backpressure
If ingestion exceeds modeling capacity:
- prioritize markets by registry priority tier
- degrade cadence (skip ticks) with explicit quality_flags
- emit lag metrics and alerts

### 10.3 Degraded mode
In degraded mode:
- reduce feature set
- increase risk_score
- emit quality_flags for consumer awareness

---

## 11. Example: Tick-Aligned Rolling Windows

Given:
- epoch_window_ms = 3600000 (1h)
- publish_cadence_ms = 30000 (30s)
- rolling windows: 5m (300000), 15m (900000), 1h (3600000)

At each tick end:
- build FeatureVectors for each rolling window ending at tick_end_ms
- run model on the primary window (e.g., 15m) with context from others
- produce ModelOutput for the tick and bundle for on-chain publishing

---

## 12. Test Plan Guidance

To validate compatibility:
- Provide fixtures of NormalizedEvents for a window.
- Run aggregation to produce FeatureVector.
- Assert deterministic features hash across runs.
- Run model stub to produce deterministic ModelOutput.
- Validate bundle hash and signature verification.

CI should include:
- unit tests for window membership
- tests for late/out-of-order handling
- determinism tests across replays
- end-to-end localnet smoke tests with commit-reveal

---

## Links

- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx
