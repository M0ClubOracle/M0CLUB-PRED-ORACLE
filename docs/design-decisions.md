
# M0Club Design Decisions (ADR-Style)

This document records major design decisions for M0Club.
It is written in an Architecture Decision Record (ADR) style so that decisions, tradeoffs, and rationale stay auditable over time.

Each decision includes:
- Context
- Decision
- Alternatives
- Consequences
- Notes / Follow-ups

---

## ADR-0001: Monorepo Layout

### Context
M0Club includes multiple components:
- Solana programs
- Off-chain engine (M0-CORE)
- API/services
- SDKs
- Infrastructure
- Docs and operational runbooks

Keeping these in separate repos increases version skew and complicates releases.

### Decision
Use a monorepo containing `programs/`, `core-engine/`, `services/`, `sdk/`, `infra/`, and `docs/`.

### Alternatives
- Multiple repos with versioned releases
- Git submodules

### Consequences
- Easier cross-component refactors and shared tooling.
- CI must handle multiple languages and build targets.
- Release process must bundle artifacts cleanly.

### Notes
- Use GitHub Actions workflows separated by domain (programs, engine, services, sdk, docker, security).

---

## ADR-0002: Commit-Reveal for Oracle Publishing

### Context
Publishing full oracle payloads directly on-chain can be copied and front-run before consumers can act. For prediction analytics, copying can cause unfair advantage and can degrade market integrity.

### Decision
Adopt a commit-reveal flow:
- Commit publishes a hash commitment to a deterministic bundle.
- Reveal publishes the payload or a proof that matches the commitment.
- Finalization selects canonical outputs per epoch.

### Alternatives
- Publish payload directly
- Use private mempools / relays only
- Encrypt payload on-chain and reveal keys later

### Consequences
- Two-phase publication increases complexity but improves integrity.
- Requires careful replay protection and deterministic hashing.
- Reveal size constraints may require merkle proofs or chunking.

### Notes
- The on-chain program should enforce ordering and epoch gating.

---

## ADR-0003: Deterministic Bundles and Stable Hashing

### Context
To verify oracle updates across different environments and SDKs, the same logical data must produce identical hashes.

### Decision
Define a canonical bundle format with:
- Stable ordering (market_id, epoch_id, outcome_id sorting)
- Fixed precision encoding for probabilities
- Strict schema versioning
- Stable hashing (e.g., SHA-256 over canonical bytes)

### Alternatives
- JSON serialization (non-deterministic ordering)
- Protobuf without canonicalization
- Hashing only a subset of fields

### Consequences
- More implementation effort but strong verifiability.
- SDKs must implement the same canonicalization rules.
- Float handling must be explicit (fixed precision or integer scaling).

### Notes
- Prefer integer-scaled probabilities in on-chain storage and hashing.

---

## ADR-0004: Signer Set Model (Off-chain Signing)

### Context
Oracle outputs must be authenticated. A single key increases compromise risk. Multi-signer sets improve robustness and enable rotation.

### Decision
Use signer sets:
- Off-chain signer agent signs bundle hashes.
- On-chain program verifies signatures against an allowed signer set.
- Signer sets can be rotated via authority/governance.
- Use replay protection (epoch + nonce/sequence) to prevent signature reuse.

### Alternatives
- Single signer key
- On-chain signing (not feasible)
- Threshold signatures (complex, possible future)

### Consequences
- Key management becomes a first-class operational concern.
- Requires robust rotation and incident response procedures.
- Signature verification costs must be considered on-chain.

### Notes
- Production should use KMS/HSM and isolate signer processes.

---

## ADR-0005: Engine + Services Separation

### Context
The engine performs ingestion and modeling. Services serve results and provide user-facing APIs. Mixing these responsibilities increases blast radius and complicates scaling.

### Decision
Separate engine from services:
- Engine: ingestion, modeling, bundling, signing, submission.
- Services: api-gateway, realtime streams, indexer, jobs.

### Alternatives
- Single monolith service
- Separate repos per component

### Consequences
- Clear scaling and security boundaries.
- More deployment units and operational complexity.
- Requires clear data contracts between components.

### Notes
- Use OpenTelemetry to connect traces across components.

---

## ADR-0006: Event Log + Feature Store

### Context
High-frequency ingestion requires replay and recovery. Modeling needs derived features and historical context.

### Decision
Use an event log (append-only) and a feature store:
- Event log for raw normalized events (replayable).
- Feature store for derived features and aggregations.
- Optional dual-write with idempotency keys.

### Alternatives
- Feature store only
- Direct streaming without persistence
- Write directly into a single relational DB only

### Consequences
- Higher operational overhead, but improved reliability and auditability.
- Backfills and reconciliation become manageable.
- Requires schema versioning and migration paths.

### Notes
- For local dev, provide a file-based event log fallback.

---

## ADR-0007: Localnet-First Developer Experience

### Context
Solana development and testing is easiest with localnet. CI should validate that on-chain programs and key flows work without relying on public networks.

### Decision
All critical flows must run on localnet:
- Anchor tests should deploy and test commit/reveal.
- Engine smoke tests should run against a local validator.
- SDK examples should be runnable locally.

### Alternatives
- Devnet-only integration tests
- Mock chain layer

### Consequences
- CI becomes more deterministic.
- Requires local validator management in workflows.
- Some edge cases still require devnet/testnet testing.

### Notes
- Provide a `make localnet` helper and documented steps.

---

## ADR-0008: Observability as a First-Class Requirement

### Context
Predictive pipelines are complex and failure can be silent (drift, degraded signal quality). Strong observability is needed to detect issues early.

### Decision
Adopt standard observability:
- OpenTelemetry traces
- Prometheus metrics
- Structured logs
- Dashboard and alert coverage for ingestion, modeling, publishing, and serving

### Alternatives
- Logs only
- Vendor-specific APM without open standards

### Consequences
- Higher baseline implementation effort but improved reliability.
- Easier multi-environment debugging.
- Enables SLO-based operations.

### Notes
- Define key SLIs for lag, error rate, latency, and drift flags.

---

## ADR-0009: Domain Coverage and Schema Strategy

### Context
M0Club spans multiple domains (sports, politics, markets). Each domain has different outcome schemas and data source reliability.

### Decision
Use a unified market model with domain-specific schemas:
- Market registry stores domain and schema type.
- Outcome schema is versioned.
- Engine connectors normalize into the same internal event shape.
- Bundles include schema_version and outcome ids.

### Alternatives
- Separate pipeline per domain
- Single fixed schema for all domains

### Consequences
- Flexibility and extensibility.
- Requires schema discipline and versioning.
- Consumers must handle schema evolution.

### Notes
- Provide schema helpers in SDKs.

---

## ADR-0010: CI as Policy Enforcement

### Context
Monorepo complexity increases risk of drift in quality and security posture.

### Decision
Use CI workflows to enforce:
- Formatting and linting (rustfmt, clippy, eslint/ruff where applicable)
- Unit and integration tests
- Dependency scanning (OSV, cargo-audit)
- Secret scanning (gitleaks)
- SAST (CodeQL)

### Alternatives
- Minimal CI only
- Manual review-only policy

### Consequences
- More setup effort, but fewer regressions and faster reviews.
- Requires maintaining pinned tool versions.
- Some workflows are best-effort when components are absent.

### Notes
- Keep workflows modular: ci.yml, programs.yml, engine.yml, services.yml, sdk.yml, docker.yml, security.yml, release.yml.

---

## Follow-ups / Future ADRs

Potential future decisions:
- Threshold signatures or MPC signer sets
- Merkle proof standardization for large bundles
- Formal verification of critical on-chain instructions
- Data source attestation and proofs of origin
- Dedicated market resolution mechanisms and dispute models

---

## Links

- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx
