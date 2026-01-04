[overview.md](https://github.com/user-attachments/files/24420706/overview.md)
# M0Club Overview

M0Club is a predictive oracle system built on Solana.
It produces real-time probability analytics for both on-chain and real-world events using a proprietary engine (M0-CORE), and delivers signed oracle outputs on-chain.

This repository is a monorepo containing:
- Solana programs (Anchor)
- M0-CORE engine (Rust)
- Off-chain services (API, indexer, realtime, jobs)
- Multi-language SDKs (TypeScript, Rust, Python)
- Cloud-native infrastructure (Docker, Kubernetes, Terraform)
- Documentation and operational runbooks

---

## What M0Club Is

M0Club is an omni-domain predictive oracle.

Traditional oracles focus on price feeds. M0Club generalizes the oracle problem to any event that can be modeled as a probabilistic outcome distribution:
- Sports match win probabilities
- Election probabilities
- Macro event risks
- Market regime probabilities
- On-chain protocol risk indicators

M0Club focuses on delivering:
- High-frequency updates
- Calibrated probability distributions
- Confidence intervals and risk metadata
- Verifiable integrity via commit-reveal and signed bundles

---

## Core Principles

### 1) The world is computed
M0Club treats events as signals that can be normalized, modeled, and converted into on-chain truth.

### 2) Omni-domain sensing
Inputs are not restricted to crypto markets. The system aggregates multi-source data across:
- On-chain state and transactions
- Sports odds and match signals
- Politics and election data
- Macro indicators and market data

### 3) Probability > raw numbers
Outputs are not a single “price.” M0Club returns probability distributions with confidence metadata and risk assessments.

### 4) Integrity by design
The system is built to resist manipulation:
- Commit-reveal prevents last-moment copying/front-running of updates
- Signer set rotation and replay protection reduce key risk
- Deterministic bundle hashing enables verification at every layer

---

## High-Level Architecture

M0Club is composed of four layers:

1. **Ingestion & Normalization**
   - Pulls data from on-chain and external sources
   - Deduplicates, validates, and canonicalizes signals

2. **Modeling & Analytics (M0-CORE)**
   - Bayesian updates, ensemble models, calibration layers
   - Produces distributions, confidence intervals, and risk scores
   - Backtesting and anomaly detection are first-class

3. **Signing & Delivery**
   - Packages analytics into deterministic bundles
   - Signs bundles with a managed signer set
   - Submits commit/reveal updates to Solana programs

4. **Serving & Observability**
   - API gateway and realtime streams for downstream consumers
   - Indexer for program events and historical analytics
   - Telemetry via OpenTelemetry / Prometheus / Grafana

---

## Repository Layout (Monorepo)

- `programs/` — Solana programs (Anchor)
  - `m0-oracle/` — commit-reveal updates, epoch finalization
  - `m0-registry/` — market registry, metadata, authority controls
  - `m0-fee-router/` — fee routing and vault logic (optional)
  - `m0-governance/` — governance / timelock primitives (optional)

- `core-engine/` — M0-CORE engine (Rust)
  - ingestion connectors, normalization, feature store, quant models
  - backtesting, anomaly detection, bundle hashing/merkle proofs
  - signer agent and tx submission utilities

- `services/` — off-chain services
  - `api-gateway/` — REST/OpenAPI access for queries and metadata
  - `indexer/` — program event indexer and storage
  - `realtime/` — websocket pub/sub for live analytics
  - `jobs/` — scheduled tasks (backfills, reconciliations)

- `sdk/` — client SDKs
  - `ts/` — TypeScript client + IDL bindings
  - `rust/` — Rust client crate
  - `python/` — Python client package

- `infra/` — deployment infrastructure
  - Docker images and Compose
  - Kubernetes manifests/Helm charts
  - Terraform modules/environments
  - Monitoring/alerts

- `docs/` — documentation
  - architecture, protocol specs, ops runbooks

---

## Oracle Output Model

M0Club outputs structured analytics for a market at a specific time window (epoch). At a minimum:

- Market identifier
- Timestamp / epoch metadata
- Distribution (e.g. probabilities for outcomes)
- Confidence interval metadata
- Risk and integrity metadata (bundle hash, signatures)

On-chain delivery is optimized for verifiability and cost:
- Commit phase publishes a commitment to a bundle hash
- Reveal phase publishes the bundle or a merkle proof path
- Finalization records canonical outputs per epoch

---

## Getting Started (Local Dev)

### Prerequisites

- Rust (stable)
- Solana CLI
- Anchor CLI
- Node.js (for program tests and TS SDK)
- Docker (optional)

### Quick Start

1) Install Solana + Anchor
- Solana: follow Solana release installer or your preferred method
- Anchor: install with `cargo install --locked anchor-cli --version <pinned>`

2) Start a local validator
```bash
solana-test-validator --reset
```

3) Build and test programs
```bash
cd programs/m0-oracle
anchor build
anchor test --skip-lint
```

4) Build engine and services
```bash
cargo build --workspace
cargo test --workspace
```

5) Run API gateway (example)
```bash
cargo run -p api-gateway
```

---

## Operational Model (Production)

A production deployment typically includes:

- Engine pods (ingestor + model pipeline)
- Signer agent pods (KMS-backed keys, replay protection)
- API gateway pods (cached queries, auth, rate limiting)
- Indexer pods (ingests on-chain events, stores analytics)
- Realtime pods (ws streams)
- Datastores (Postgres/ClickHouse/Redis)
- Monitoring stack (Prometheus/Grafana/Loki)

Infrastructure examples:
- `infra/docker/compose.dev.yml` for local integration
- `infra/k8s/` + Helm for production clusters
- `infra/terraform/` for cloud resources

---

## Security Posture

M0Club uses layered controls:

- Commit-reveal update flow on-chain
- Deterministic bundle hashing and optional merkle proofs
- Signer set rotation and guardianship controls
- Replay protection and submission idempotency
- Strict config validation and secrets separation
- Continuous scanning in CI (SAST, dependency, secret scanning)

If you suspect a vulnerability, do not open a public issue.
Follow `SECURITY.md`.

---

## Roadmap (High Level)

- Expand domain connectors and normalization rules
- Improve calibration and drift detection
- Provide richer market metadata and risk explanations
- Harden signer operations (KMS, HSM support)
- Improve indexing and realtime latency
- Expand SDK coverage and examples

---

## Links

- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx
