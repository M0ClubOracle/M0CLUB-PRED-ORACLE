
# M0CORE Architecture

This document describes the end-to-end architecture of M0Club, including on-chain programs, off-chain engine components, service topology, data contracts, and operational concerns.

M0Club is an omni-domain predictive oracle built on Solana. It ingests multi-source signals, produces calibrated probability analytics, packages them into verifiable bundles, and publishes commitments and reveals on-chain.

---

## 1. System Goals

### Primary goals
- Deliver real-time probability distributions for markets (on-chain and real-world events).
- Provide verifiable integrity for every oracle update.
- Maintain low latency and high throughput on Solana.
- Support multiple consumers (protocols, apps, traders, dashboards) via SDKs and APIs.

### Non-goals
- Being a generalized data warehouse for external datasets.
- Exposing raw proprietary signals directly on-chain.
- Guaranteeing outcomes beyond the limits of the underlying data sources.

---

## 2. Architecture at a Glance

M0Club consists of four layers:

1) **Ingestion & Normalization**
- Connectors ingest on-chain and external signals.
- Signals are validated, normalized, deduplicated, timestamped.
- Outputs are stored in a feature store / event log.

2) **Modeling & Analytics (M0-CORE)**
- Bayesian/ensemble modeling produces distributions.
- Calibration layers estimate confidence intervals and uncertainty.
- Drift/anomaly detection guards against feed manipulation and model failure.

3) **Bundle Signing & Delivery**
- Analytics are converted into deterministic bundles.
- Bundles are hashed and signed by a signer set.
- Commit-reveal publishes commitment and then reveal/proofs on-chain.

4) **Serving & Observability**
- API gateway and realtime streams serve analytics and metadata.
- Indexer stores on-chain events and reconciles the published outputs.
- Monitoring and alerting ensure production reliability.

---

## 3. Component Topology

### 3.1 On-chain programs (Anchor)

Typical set of programs:

- **m0-oracle**
  - Market epochs and update windows
  - Commit accounts and reveal accounts
  - Finalization logic (canonical output per epoch)
  - Replay protection and authority checks
  - Optional merkle proof verification for large bundles

- **m0-registry**
  - Market metadata registry (symbol, domain, outcome schema)
  - Authority, guardians, signer set references
  - Configuration pointers (epoch schedule, fees, update cadence)

- **m0-fee-router** (optional)
  - Fee routing to treasury, buyback, incentives
  - Vault logic and distribution policy

- **m0-governance** (optional)
  - Timelock/guardian actions
  - Parameter changes and emergency controls

### 3.2 Off-chain engine (core-engine)

Typical subcomponents:

- **m0-ingestor**
  - Connectors: on-chain state, orderbooks, odds feeds, election data, macro feeds
  - Normalization rules and schema versioning
  - Deduplication (idempotency keys), watermarking, ordering
  - Writes to event log + feature store

- **m0-quant**
  - Feature extraction and aggregation windows
  - Bayesian updates / ensemble models
  - Calibration and uncertainty estimation
  - Backtesting and offline evaluation pipeline

- **m0-bundler**
  - Converts model outputs into canonical bundle format
  - Deterministic serialization (stable ordering, stable rounding)
  - Hashing (bundle hash, optional merkle root)

- **m0-signer-agent**
  - Holds signer keys (prefer KMS/HSM in production)
  - Signs bundles (or merkle roots)
  - Enforces replay protections (sequence numbers, epoch gating)

- **m0-submitter**
  - Sends commit transactions and reveal transactions
  - Retries with idempotency and blockhash refresh
  - Confirms finality and records status to storage

### 3.3 Services

- **api-gateway**
  - REST API for market metadata and latest analytics
  - Auth / rate limiting for public endpoints
  - Caching layer (Redis) for hot reads

- **realtime**
  - Websocket pub/sub for market updates
  - Fanout and backpressure management

- **indexer**
  - Subscribes to Solana program events (logs/webhooks)
  - Stores commits, reveals, finalizations
  - Reconciles off-chain bundles with on-chain commitments

- **jobs**
  - Backfill tasks, reconciliation tasks
  - Periodic health checks, drift reports

### 3.4 Infrastructure

- Docker images for each component
- Compose for local integration
- Kubernetes/Helm for production deployments
- Terraform for cloud resources and IAM
- Monitoring stack (Prometheus/Grafana/Loki)

---

## 4. Data Flow (End-to-End)

### 4.1 Ingestion
1. Connector polls/streams source events.
2. Event passes schema validation and normalization.
3. Event is assigned:
   - `source_id`
   - `event_id` (stable hash)
   - `ts_ingested`
   - `ts_event`
   - `quality_flags`
4. Event is written to the event log and feature store.

### 4.2 Modeling
1. Feature windows are constructed by market and time bucket.
2. Models compute outcome distributions:
   - probabilities
   - confidence interval metadata
   - risk scores
3. Drift/anomaly checks run:
   - distribution shift
   - feed divergence across sources
   - outlier detection
4. Outputs are persisted as `ModelOutput` objects.

### 4.3 Bundling
1. `ModelOutput` is serialized into a canonical bundle.
2. Bundle is hashed to produce `bundle_hash`.
3. Optional: compute merkle root if bundle is large or multi-market.

### 4.4 Signing
1. Signer agent signs the `bundle_hash` (and/or merkle root).
2. Signature metadata includes:
   - signer public key
   - signature
   - sequence/nonce
   - epoch id

### 4.5 On-chain publish (commit-reveal)
1. Commit tx publishes:
   - market id
   - epoch id
   - bundle_hash (commitment)
   - signer set id / signer pubkey reference
2. Reveal tx publishes:
   - bundle payload OR merkle proof path
   - signature(s)
3. Finalization records canonical result for epoch.

### 4.6 Serving
1. Indexer records the on-chain commit/reveal/finalize events.
2. API/realtime serves:
   - latest finalized distribution
   - pending commits (optional)
   - metadata and historical data

---

## 5. Data Contracts

This section defines the minimal contracts used between components.

### 5.1 Canonical identifiers

- **MarketId**
  - stable string or u64 derived from registry
  - example: `POLITICS_US_PRES_2028`
- **EpochId**
  - monotonic identifier or (market_id, window_start_ts)
- **BundleHash**
  - `sha256(canonical_bytes)` or equivalent stable hash

### 5.2 Off-chain bundle schema (logical)

`OracleBundle` (logical fields):
- `schema_version`
- `market_id`
- `epoch_id`
- `ts_produced`
- `outcomes[]`
  - `outcome_id`
  - `p` (probability)
  - `ci_low`
  - `ci_high`
  - `risk_score`
- `model_meta`
  - calibration version
  - drift flags
- `integrity`
  - `bundle_hash`
  - `signatures[]`

Canonical serialization requirements:
- stable key ordering
- stable float encoding (fixed precision)
- stable outcome ordering by `outcome_id`

### 5.3 On-chain accounts (conceptual)

Typical `m0-oracle` accounts:
- `Market`
- `Epoch`
- `Commitment`
- `Reveal`
- `SignerSet` (or registry reference)
- `Authority` / `Guardian`

Actual account layouts are defined by the on-chain programs.

---

## 6. Integrity and Threat Model

### 6.1 Threats
- Feed manipulation and adversarial inputs
- Copying and front-running updates
- Signer key compromise
- Replay attacks and duplicated submissions
- Source outage and partial data corruption

### 6.2 Controls
- Commit-reveal prevents last-moment copying of full payloads.
- Deterministic hashing enables verification across layers.
- Signer sets can be rotated; keys stored in KMS/HSM.
- Replay protection: sequence numbers, epoch gating, idempotency keys.
- Multi-source divergence checks to detect manipulation.
- Strict config validation; secrets never committed.

---

## 7. Operational Concerns

### 7.1 Latency targets
- Ingestion to model output: milliseconds to seconds (domain-dependent)
- Commit submission: seconds (network conditions)
- Reveal and finalize: depends on configured cadence and confirmations

### 7.2 Throughput
- Ingestion scales horizontally by connector shards.
- Modeling scales per market partition and time buckets.
- Realtime stream scales by topic partitioning and fanout.

### 7.3 Storage
Common choices:
- Event log: Kafka/Redpanda or NATS JetStream
- Feature store: Postgres + Timescale, or ClickHouse
- Cache: Redis

### 7.4 Observability
- Traces: OpenTelemetry
- Metrics: Prometheus
- Logs: Loki or ELK
- Dashboards: Grafana

Key metrics:
- ingestion lag
- model latency and success rate
- drift/anomaly flags
- commit/reveal success rate
- indexer lag
- api error rates
- realtime fanout backpressure

---

## 8. Local Development Architecture

Local setup typically includes:
- `solana-test-validator`
- programs built and deployed via Anchor
- engine running in a local profile with mocked connectors
- api-gateway and realtime for inspection
- optional docker compose for Postgres/Redis

Suggested local profile defaults:
- short epoch window
- relaxed rate limits
- verbose logging
- local file-based event log fallback

---

## 9. Deployment Architecture (Production)

Typical production cluster:
- dedicated namespace per environment (dev/staging/prod)
- separate signer isolation boundary
- private network policies for engine and services
- public ingress only for api-gateway and realtime
- strict secret management and key rotation process
- canary deployments for model and engine changes

---

## 10. Change Management

- Schema changes require version bumps and backward compatibility.
- On-chain program upgrades require governance approval and migration plan.
- Signer set rotations require coordinated rollout across engine and programs.
- CI enforces security scanning and minimal quality gates.

---

## 11. Appendix: Reference Diagrams

### 11.1 End-to-end pipeline

```text
Sources (On-chain + External)
        |
        v
+-------------------------+
| Ingestion & Normalize   |
|  - connectors           |
|  - validation           |
|  - dedupe               |
+-------------------------+
        |
        v
+-------------------------+
| M0-CORE Modeling        |
|  - features             |
|  - bayesian/ensemble    |
|  - calibration          |
|  - anomaly checks       |
+-------------------------+
        |
        v
+-------------------------+
| Bundle + Sign           |
|  - canonical encoding   |
|  - bundle hash          |
|  - signatures           |
+-------------------------+
        |
        v
+-------------------------+
| Solana Commit/Reveal    |
|  - commit tx            |
|  - reveal tx/proofs     |
|  - finalize epoch       |
+-------------------------+
        |
        v
+-------------------------+
| Indexer + Serving       |
|  - api gateway          |
|  - realtime streams     |
|  - historical store     |
+-------------------------+
```

---

## Links

- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx
