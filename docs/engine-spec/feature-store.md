
# Feature Store Specification (M0-CORE)

This document specifies the feature store subsystem for M0Club (M0-CORE).
The feature store persists computed FeatureVectors and related metadata so that models, backtests, SDK verifiers, and operational dashboards can access consistent, versioned features across time.

This spec defines:
- feature store goals and responsibilities
- data model and schemas
- write and read paths
- determinism and versioning rules
- retention and compaction
- APIs for online and offline use
- security, observability, and testing

---

## 1. Goals

Primary goals:
- Persist FeatureVectors keyed by deterministic window identifiers.
- Provide fast online reads for real-time modeling and dashboards.
- Provide efficient offline reads for backtesting and research.
- Enforce schema versioning and reproducibility.
- Support retention policies and cost controls.

Non-goals:
- Storing raw source data permanently (handled by ingestion log storage).
- Serving as a general data lake for all proprietary datasets.

---

## 2. Responsibilities

The feature store is responsible for:
- storing FeatureVectors produced by the aggregator
- storing feature schema metadata and version history
- indexing by market_id, epoch_id, tick_index, window bounds
- providing query APIs for:
  - latest features for a market
  - range queries for backtests
  - feature schema discovery
- ensuring features are immutable once committed (append-only semantics)

---

## 3. Storage Backends

M0Club can use multiple backends depending on environment:

### 3.1 Online store (low latency)
Options:
- Postgres (recommended for v1 simplicity)
- Redis (cache layer on top of Postgres)
- ClickHouse (also good for analytics reads)

### 3.2 Offline store (large history)
Options:
- ClickHouse (recommended)
- Parquet files on object storage (S3/GCS) with partitioning
- Delta/Iceberg style tables (advanced)

Recommended v1:
- Postgres for online + ClickHouse for offline analytics (optional)

Local/dev:
- SQLite or Postgres + file dumps for easy replay.

---

## 4. Data Model

### 4.1 FeatureVector record
A FeatureVector is the canonical output of aggregation.

Required fields:
- `window_key` (32 bytes / hex) deterministic hash
- `market_id` (string)
- `epoch_id` (u64 or string)
- `tick_index` (u32)
- `window_start_ms` (u64)
- `window_end_ms` (u64)
- `feature_schema_version` (u16)
- `features_blob` (bytes) canonical encoding of feature map
- `features_hash` (32 bytes) sha256 of canonical features_blob
- `quality_flags` (u32)
- `coverage_json` (json) coverage counts
- `created_at_ms` (u64)
- `producer_instance` (string) optional
- `trace_id` (string) optional

### 4.2 Features encoding
Features_blob MUST be deterministic and versioned.

Recommended encoding:
- fixed list of features per schema version
- each value stored in fixed-point integers or primitives
- stable ordering defined by schema

Avoid:
- arbitrary maps with unstable ordering unless canonicalized strictly

### 4.3 FeatureSchema table
Feature schema metadata should be stored separately.

Fields:
- `feature_schema_version` (u16)
- `domain` (string/enum)
- `schema_hash` (32 bytes)
- `created_at_ms`
- `features[]` definitions (stored as JSON or normalized tables)

schema_hash:
- sha256 of canonical schema JSON representation

### 4.4 Partitioning keys
Indexes should support:
- by market_id
- by (market_id, window_end_ms) for latest reads
- by (market_id, feature_schema_version, window_end_ms) for stable model inputs
- by epoch_id and tick_index for correlation with oracle publishing

---

## 5. Write Path

### 5.1 Producer
The aggregator emits FeatureVectors.
The feature store writer:
- validates FeatureVector schema version exists
- computes features_hash if not present
- computes window_key if not present
- writes record with upsert or append-only semantics

### 5.2 Idempotency
Writes MUST be idempotent by window_key.
If the same window_key is written twice:
- the second write should be rejected or treated as no-op
- if correction is needed, a new schema/version should be produced (immutable history)

Recommended:
- enforce UNIQUE constraint on window_key

### 5.3 Atomicity
FeatureVector writes should be atomic:
- either the entire record is stored or nothing is stored
- use transactions where available

### 5.4 Validation checks
- market_id canonical rules
- window bounds consistent
- feature_schema_version known
- features_hash matches features_blob

---

## 6. Read Path

### 6.1 Online reads (serving)
API patterns:
- `get_latest_features(market_id, schema_version)`
- `get_features_for_window(market_id, window_start_ms, window_end_ms, schema_version)`
- `get_features_by_epoch(market_id, epoch_id, tick_index)`
- `list_feature_schemas(domain)`

Latency goals:
- p50 < 20ms for latest reads (with cache)
- p95 < 100ms

### 6.2 Offline reads (analytics/backtest)
Range queries:
- `query_features(market_id, start_ms, end_ms, schema_version)`
- allow selection of feature subsets (projection)
- allow downsampling or aggregation for long ranges

ClickHouse is ideal for time-series range scans.

---

## 7. Versioning and Reproducibility

### 7.1 Feature schema versioning
Models must declare which feature_schema_version they consume.
Changing feature semantics or scale factors requires a new schema version.

### 7.2 Backward compatibility
Consumers should:
- reject unknown schema versions unless explicitly supported
- use schema_hash to ensure schema integrity

### 7.3 Immutable records
FeatureVectors are immutable once written.
Corrections are handled by:
- producing a new FeatureVector under a new window_key derived from new schema/version or correction policy
- recording correction relationships (optional)

---

## 8. Retention and Compaction

### 8.1 Retention policy
Different markets can have different retention requirements.

Recommended policy tiers:
- hot: last 7-30 days in Postgres
- warm: last 6-12 months in ClickHouse
- cold: archives in object storage

### 8.2 Compaction
For long-term storage:
- keep full cadence for recent period
- downsample older data (e.g., 30s -> 5m aggregates) if allowed
- store downsampled features under a different schema/version

Compaction must preserve determinism and be documented.

---

## 9. Caching Strategy

Use a cache for hot reads:
- Redis keyed by (market_id, schema_version, latest_window_end_ms)
- TTL based on publish cadence
- invalidate on new writes

Cache should store:
- features_blob
- features_hash
- metadata (window bounds)

---

## 10. Integrity and Verification

### 10.1 features_hash usage
features_hash can be included in oracle bundles as a commitment:
- proves that the model used a specific feature snapshot
- does not reveal raw features publicly

### 10.2 Verification routine
A verifier can:
- fetch FeatureVector for a window
- recompute features_hash from canonical features_blob
- compare to included features_hash in oracle bundle

This allows integrity checks without exposing proprietary data.

---

## 11. Security

- Restrict write access to engine components only.
- Read access can be tiered:
  - public: limited metadata (window bounds, hashes, quality flags)
  - internal: full features_blob
- Encrypt at rest where possible.
- Do not log full features_blob in production logs.
- Apply row-level filtering for sensitive markets if needed.

---

## 12. Observability

Expose metrics:
- feature_vectors_written_total
- feature_store_write_latency_ms
- feature_store_read_latency_ms
- cache_hit_rate
- uniqueness_violation_count
- schema_version_mismatch_count

Logs include:
- market_id
- epoch_id
- window_end_ms
- schema_version
- features_hash
- trace_id

---

## 13. Reference SQL (Postgres) (Conceptual)

```sql
CREATE TABLE feature_vectors (
  window_key BYTEA PRIMARY KEY,
  market_id TEXT NOT NULL,
  epoch_id BIGINT NOT NULL,
  tick_index INTEGER NOT NULL,
  window_start_ms BIGINT NOT NULL,
  window_end_ms BIGINT NOT NULL,
  feature_schema_version SMALLINT NOT NULL,
  features_blob BYTEA NOT NULL,
  features_hash BYTEA NOT NULL,
  quality_flags INTEGER NOT NULL,
  coverage_json JSONB NOT NULL,
  created_at_ms BIGINT NOT NULL
);

CREATE INDEX idx_feature_vectors_market_end
  ON feature_vectors (market_id, window_end_ms DESC);

CREATE INDEX idx_feature_vectors_market_schema_end
  ON feature_vectors (market_id, feature_schema_version, window_end_ms DESC);
```

Note:
- Actual schema may vary; this section is guidance for implementation.

---

## 14. Test Plan

Unit tests:
- deterministic encoding and hashing of features_blob
- window_key derivation stability
- idempotent write behavior under retries
- schema version enforcement

Integration tests:
- aggregator -> feature store write -> model read -> bundle
- cache behavior for latest reads
- range query correctness

Load tests:
- sustained write rate at target cadence
- query p95 latency under load

---

## Links

- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx
