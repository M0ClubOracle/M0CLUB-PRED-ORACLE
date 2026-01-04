
# M0-CORE Engine Specification

This document specifies the M0-CORE engine layer for M0Club.
M0-CORE is responsible for turning high-volume multi-source signals into deterministic, verifiable oracle bundles suitable for commit-reveal publishing on Solana.

This spec is implementation-oriented and maps to the monorepo structure under `core-engine/` and related services under `services/`.

---

## 1. Goals

Primary goals:
- Ingest multi-source signals with strong validation and schema discipline.
- Produce calibrated probability distributions and uncertainty metadata.
- Generate deterministic bundles with stable hashing and signing.
- Publish updates via commit-reveal and reconcile results.
- Support low-latency real-time analytics and high throughput.

Non-goals:
- Storing proprietary raw datasets permanently in public systems.
- Implementing every domain-specific model inside this document (models are pluggable).

---

## 2. Engine Responsibilities

M0-CORE consists of these responsibility planes:

1) **Ingestion**
- Connectors ingest raw inputs from on-chain protocols and external sources.
- Normalize into `NormalizedEvent` envelopes.
- Deduplicate, watermark, validate, and route to partitions.

2) **Aggregation**
- Build tick-aligned and rolling modeling windows.
- Compute feature vectors with fixed-point determinism.
- Track coverage and data quality flags.

3) **Modeling**
- Consume FeatureVectors and produce ModelOutputs:
  - outcome probability distribution
  - confidence interval metadata
  - risk score
  - explanation references (non-sensitive)
- Run calibration and drift detection layers.

4) **Bundling**
- Canonicalize ModelOutputs into the oracle output format.
- Produce canonical bytes and compute bundle hash.
- Optionally compute Merkle root for large bundles.

5) **Signing**
- Request signatures from signer agent(s) using allocated sequence numbers.
- Assemble integrity metadata and attach signatures.

6) **Publishing**
- Submit commit transactions and reveal transactions to Solana.
- Confirm finality, retry idempotently, and persist state.

7) **Reconciliation**
- Validate on-chain commitments against local records.
- Detect missing reveals, stale sequences, and anomalies.
- Trigger correction workflows if configured.

---

## 3. Component Breakdown

Recommended component modules under `core-engine/`:

- `m0-ingestor`
- `m0-aggregator`
- `m0-quant`
- `m0-bundler`
- `m0-signer-client`
- `m0-submitter`
- `m0-reconciler`
- `m0-common` (shared types, canonical encoding, hashing)

### 3.1 m0-ingestor
Responsibilities:
- Define connectors for each data source.
- Normalize events into a stable internal schema.
- Guarantee idempotency using `event_id`.
- Emit normalized events into an event log.

Key features:
- per-source rate limiting and circuit breakers
- schema validation and versioning
- quality flagging and coverage stats

Outputs:
- `NormalizedEvent` stream keyed by market_id partition.

### 3.2 m0-aggregator
Responsibilities:
- Derive epoch/tick boundaries from registry parameters.
- Maintain aggregation state per window.
- Compute FeatureVectors and features_hash commitments.

Key features:
- out-of-order handling within allowed lateness
- deterministic replay ordering `(event_time_ms, event_id)`
- rolling windows (5m, 15m, 1h) aligned to tick boundaries

Outputs:
- `FeatureVector` stream keyed by market_id and window key.

### 3.3 m0-quant
Responsibilities:
- Run modeling and probability analytics on FeatureVectors.
- Maintain model registry and per-market model selection.
- Perform calibration and drift checks.

Key features:
- pluggable model interface
- backtesting hooks (offline mode)
- risk scoring and quality flag propagation

Outputs:
- `ModelOutput` per market/tick/epoch.

### 3.4 m0-bundler
Responsibilities:
- Build OracleBundle objects from ModelOutputs.
- Enforce canonical ordering and fixed-point encoding.
- Compute bundle_id, bundle_hash, and optional merkle roots.

Key features:
- stable serialization across languages
- schema_version enforcement
- bundle sizing checks for reveal limits

Outputs:
- `OracleBundle` with canonical bytes and hash.

### 3.5 m0-signer-client
Responsibilities:
- Allocate sequence values per replay policy.
- Request signatures from signer agent(s) (mTLS auth).
- Assemble signature metadata sorted by pubkey bytes.

Key features:
- retry with idempotency keys
- durable sequence allocator integration
- audit log correlation ids

Outputs:
- signed bundles ready for publishing.

### 3.6 m0-submitter
Responsibilities:
- Submit commit and reveal transactions.
- Confirm via RPC, handle retries with blockhash refresh.
- Maintain publish state machine per (market, epoch).

Key features:
- idempotent tx building keyed by (epoch, bundle_hash)
- concurrency controls
- dynamic fee and compute budget tuning

Outputs:
- on-chain tx signatures, status updates, persistent records.

### 3.7 m0-reconciler
Responsibilities:
- Subscribe to on-chain logs and indexer outputs.
- Reconcile expected vs observed commit/reveal/finalize.
- Trigger alerts, re-submissions, and corrections.

Key features:
- detect missing reveal after max delay
- detect sequence monotonicity issues
- detect mismatched signer set id

Outputs:
- reconciliation reports, correction triggers.

### 3.8 m0-common
Shared crates/modules:
- canonical encoding for OracleBundle
- hashing utilities
- time window derivation
- schema validation helpers
- error types and tracing helpers

---

## 4. Data Contracts

### 4.1 NormalizedEvent
Fields (summary):
- event_id (32 bytes)
- source_id
- market_id
- event_time_ms
- ingest_time_ms
- payload_type + payload
- quality_flags
- schema_version

### 4.2 FeatureVector
Fields (summary):
- market_id, epoch_id, tick_index
- window_start_ms, window_end_ms
- feature_schema_version
- features (fixed-point values)
- coverage object
- quality_flags
- features_hash (optional)

### 4.3 ModelOutput
Fields (summary):
- market_id, epoch_id, tick_index
- outcomes[] with p_scaled and ci bounds
- risk_score
- quality_flags
- model_id, model_version
- explanations refs (optional)

### 4.4 OracleBundle
As defined in `docs/protocol-spec/oracle-output-format.md`.

---

## 5. Determinism and Canonicalization

M0-CORE MUST ensure:
- time windows derived from registry params (origin_ms, epoch_window_ms, cadence)
- fixed-point probability scaling
- stable sorting of bundle items
- canonical bytes used for hashing and signing
- bundle_hash computed with integrity.bundle_hash zeroed before hashing

All SDKs must be able to:
- re-serialize the bundle and recompute hash
- verify signatures

---

## 6. Config and Profiles

M0-CORE should support layered config:
- base config files (YAML/TOML)
- environment variables
- per-market overrides via registry
- runtime flags for local profiles

Recommended profiles:
- `local`
- `dev`
- `staging`
- `prod`

Config domains:
- RPC endpoints and commitment levels
- event log backend (file, nats, kafka)
- storage backend (postgres/clickhouse)
- signer agent endpoints and auth
- publish cadence and concurrency
- safety thresholds (staleness bounds, lateness, drift thresholds)

---

## 7. Engine Runtime Topology

Typical runtime services:
- ingestor workers (sharded by source and market)
- aggregator workers (sharded by market partitions)
- quant workers (sharded by market partitions)
- bundler (can be co-located with quant)
- signer client (isolated boundary)
- submitter (isolated boundary)
- reconciler (subscribes to chain + indexer)

A single binary can embed multiple roles in local mode; production typically separates roles.

---

## 8. Publish State Machine

For each (market_id, epoch_id):
- `READY` (model output available)
- `BUNDLED` (bundle hash computed)
- `SIGNED` (signatures attached)
- `COMMITTED` (commit tx confirmed)
- `REVEALED` (reveal tx confirmed)
- `FINALIZED` (finalize tx confirmed or observed)
- `FAILED` (terminal error) with retry policy

The submitter must maintain idempotency:
- if COMMITTED exists for bundle_hash, do not allocate new sequence for same hash
- if REVEALED exists, do not resubmit unless confirmation missing
- use reveal PDA and commit PDA checks to ensure safe retries

---

## 9. Observability

M0-CORE must emit:
- structured logs (JSON)
- traces (OpenTelemetry)
- metrics (Prometheus)

Key metrics:
- ingestion lag (event_time vs ingest_time)
- events per second per connector
- aggregation window completion time
- model latency per market
- drift/anomaly flags frequency
- bundle size and reveal mode usage
- commit/reveal success rate and retries
- RPC error rates and latency
- sequence allocation monotonicity

---

## 10. Security

### 10.1 Key handling
- signer keys must live behind signer agent boundary
- use KMS/HSM in production
- do not store plaintext keys in configs or repo

### 10.2 Secrets
- all secrets are injected via secret manager
- CI must scan for leaked secrets
- local dev uses `.env` with `.gitignore`

### 10.3 Network isolation
- signer agent and submitter should be isolated
- only allow required outbound RPC and internal calls
- apply strict mTLS and auth to signer API

---

## 11. Local Development

Local workflow:
1) start `solana-test-validator`
2) deploy programs with Anchor
3) run engine in `local` profile with mock connectors
4) run api-gateway and realtime services for inspection

Local environment should include:
- file-based event log backend
- sqlite/postgres optional storage
- deterministic test fixtures for events and model outputs

---

## 12. Production Deployment

Production recommendations:
- separate namespaces per env
- horizontal scaling for ingestion and modeling workers
- canary deployments for quant and bundler changes
- strict alerting for missing reveal and sequence issues
- runbook-driven incident response integration

---

## 13. Test Plan

Engine tests should include:
- deterministic canonicalization tests (hash stable across runs)
- aggregation determinism under replay
- model interface contract tests
- signing message tests and signature verification tests
- localnet end-to-end: commit + reveal + finalize

Provide test vectors in SDKs to guarantee cross-language compatibility.

---

## Links

- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx
