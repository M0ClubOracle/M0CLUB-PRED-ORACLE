
# M0Club — Deployment Guide (End-to-End)

This guide is an end-to-end, operator-grade deployment document for the M0Club monorepo.
It is written to be actionable: commands, environment variables, expected outputs, validation checks,
rollback steps, and troubleshooting are included.

> IMPORTANT
> - Never commit secrets to Git.
> - Production signers should use KMS/HSM-backed keys.
> - On-chain program upgrades must be audited and staged before production promotion.

---

## Table of Contents

- [1. Overview](#1-overview)
  - [1.1 What You Deploy](#11-what-you-deploy)
  - [1.2 Deployment Targets](#12-deployment-targets)
  - [1.3 Environments](#13-environments)
- [2. Prerequisites](#2-prerequisites)
  - [2.1 Tooling](#21-tooling)
  - [2.2 Accounts and Access](#22-accounts-and-access)
  - [2.3 Host Requirements](#23-host-requirements)
- [3. Repository and Build Artifacts](#3-repository-and-build-artifacts)
  - [3.1 Monorepo Layout](#31-monorepo-layout)
  - [3.2 Versioning Strategy](#32-versioning-strategy)
  - [3.3 Images and Tags](#33-images-and-tags)
- [4. Configuration Model](#4-configuration-model)
  - [4.1 Config Files](#41-config-files)
  - [4.2 Environment Variables](#42-environment-variables)
  - [4.3 Secrets](#43-secrets)
  - [4.4 Recommended Secret Manager Patterns](#44-recommended-secret-manager-patterns)
- [5. Local Deployment (Docker Compose)](#5-local-deployment-docker-compose)
  - [5.1 Quickstart](#51-quickstart)
  - [5.2 Local Solana Validator (Optional)](#52-local-solana-validator-optional)
  - [5.3 Local Environment Variables](#53-local-environment-variables)
  - [5.4 Validation](#54-validation)
  - [5.5 Troubleshooting Local](#55-troubleshooting-local)
- [6. Staging Deployment (Kubernetes)](#6-staging-deployment-kubernetes)
  - [6.1 Cluster Baseline](#61-cluster-baseline)
  - [6.2 Namespaces](#62-namespaces)
  - [6.3 Ingress and TLS](#63-ingress-and-tls)
  - [6.4 Datastores](#64-datastores)
  - [6.5 Deploy via Helm](#65-deploy-via-helm)
  - [6.6 Validation](#66-validation)
- [7. Production Deployment (Kubernetes)](#7-production-deployment-kubernetes)
  - [7.1 Production Topology](#71-production-topology)
  - [7.2 Resource Sizing](#72-resource-sizing)
  - [7.3 Autoscaling](#73-autoscaling)
  - [7.4 High Availability](#74-high-availability)
  - [7.5 Disaster Recovery](#75-disaster-recovery)
- [8. On-Chain Program Deployment](#8-on-chain-program-deployment)
  - [8.1 Program Build](#81-program-build)
  - [8.2 Program IDs and Registry](#82-program-ids-and-registry)
  - [8.3 Deploy to Devnet/Testnet/Mainnet](#83-deploy-to-devnettestnetmainnet)
  - [8.4 Upgrade Policy](#84-upgrade-policy)
  - [8.5 IDL Generation and Publishing](#85-idl-generation-and-publishing)
  - [8.6 Verifying On-Chain State](#86-verifying-on-chain-state)
- [9. Engine Deployment](#9-engine-deployment)
  - [9.1 Engine Roles](#91-engine-roles)
  - [9.2 Ingestion Connectors](#92-ingestion-connectors)
  - [9.3 Feature Store Backends](#93-feature-store-backends)
  - [9.4 Bundling and Hashing](#94-bundling-and-hashing)
  - [9.5 Signer Agent](#95-signer-agent)
- [10. Services Deployment](#10-services-deployment)
  - [10.1 API Gateway](#101-api-gateway)
  - [10.2 Indexer](#102-indexer)
  - [10.3 Realtime WebSocket](#103-realtime-websocket)
  - [10.4 Jobs](#104-jobs)
  - [10.5 Dashboard](#105-dashboard)
- [11. Database and Migrations](#11-database-and-migrations)
  - [11.1 Postgres](#111-postgres)
  - [11.2 ClickHouse (Optional)](#112-clickhouse-optional)
  - [11.3 Redis](#113-redis)
  - [11.4 Migration Workflow](#114-migration-workflow)
- [12. Observability](#12-observability)
  - [12.1 Metrics](#121-metrics)
  - [12.2 Logs](#122-logs)
  - [12.3 Tracing (Optional)](#123-tracing-optional)
  - [12.4 SLOs and Alerts](#124-slos-and-alerts)
- [13. Security Hardening](#13-security-hardening)
  - [13.1 Network Policy](#131-network-policy)
  - [13.2 RBAC](#132-rbac)
  - [13.3 Image Signing and SBOM](#133-image-signing-and-sbom)
  - [13.4 Dependency Scanning](#134-dependency-scanning)
  - [13.5 Secrets Rotation](#135-secrets-rotation)
- [14. Release, Promotion, and Rollback](#14-release-promotion-and-rollback)
  - [14.1 CI/CD Flow](#141-cicd-flow)
  - [14.2 Staging Promotion](#142-staging-promotion)
  - [14.3 Production Cutover](#143-production-cutover)
  - [14.4 Rollback Procedures](#144-rollback-procedures)
- [15. Verification Checklist](#15-verification-checklist)
- [16. Troubleshooting](#16-troubleshooting)
  - [16.1 Common Failure Modes](#161-common-failure-modes)
  - [16.2 Debug Commands](#162-debug-commands)
- [17. Appendix](#17-appendix)
  - [17.1 Example Environment Variable Set](#171-example-environment-variable-set)
  - [17.2 Example Helm Values Skeleton](#172-example-helm-values-skeleton)
  - [17.3 Example Kubernetes Secret Manifests](#173-example-kubernetes-secret-manifests)

---

## 1. Overview

### 1.1 What You Deploy

M0Club is deployed as an integrated system:

- **On-chain programs** (Anchor):
  - `m0-oracle` (epochs, commit-reveal, finalization)
  - `m0-registry` (market registry, metadata)
  - `m0-fee-router` (routing vaults and distribution rules)
  - `m0-governance` (timelock/governor scaffolding)
- **Core engine** (Rust workspace):
  - ingestion, normalization, feature-store, quant, anomaly, bundle, signer, runtime
  - daemons: `m0d`, `m0-ingestd`, `m0-backtestd`, `m0-signer-agent`
- **Services**:
  - `api-gateway` (REST + OpenAPI)
  - `indexer` (program event indexing)
  - `realtime` (WebSocket feeds)
  - `jobs` (maintenance scheduler)
  - `dashboard` (Next.js)
- **Infra/Observability**:
  - docker-compose for local
  - Kubernetes/Helm for staging/prod
  - Terraform scaffolding (optional)
  - Prometheus/Grafana/Loki

### 1.2 Deployment Targets

You typically deploy to:
- **Local**: developer machines (docker-compose)
- **Staging**: Kubernetes cluster with reduced throughput
- **Production**: Kubernetes cluster with autoscaling + hardened signer architecture

### 1.3 Environments

Use the environment name consistently:
- `dev`
- `staging`
- `prod`

Each environment must have isolated:
- Solana RPC endpoints and program deployments
- signer keys and signer sets
- databases and caches
- domain ingestion credentials
- telemetry backends

---

## 2. Prerequisites

### 2.1 Tooling

Minimum:
- Docker 24+ and Docker Compose v2
- kubectl 1.27+ (for k8s)
- Helm 3.13+
- Rust (pinned by `rust-toolchain.toml`)
- Node.js 20+
- Python 3.10+

Optional:
- k6 (load tests)
- Terraform (cloud provisioning)
- Cosign (image signing)

### 2.2 Accounts and Access

You need access to:
- a container registry (GHCR/ECR/GCR/ACR)
- Kubernetes cluster admin (staging/prod)
- DNS provider (for ingress)
- a Solana RPC provider (devnet/testnet/mainnet)
- secret manager (recommended) or K8s secrets (minimum)

### 2.3 Host Requirements

Local dev host recommended:
- 8+ CPU cores
- 16+ GB RAM
- 50+ GB disk
- stable internet for pulling images and fetching public RPC data

Staging/prod requirements depend on ingestion throughput and model complexity.

---

## 3. Repository and Build Artifacts

### 3.1 Monorepo Layout

```text
m0club/
  programs/
  core-engine/
  services/
  sdk/
  infra/
  docs/
  scripts/
  tests/
  config/
```

### 3.2 Versioning Strategy

Recommended:
- single repo release tag: `vX.Y.Z`
- docker images tagged with both:
  - immutable commit SHA: `sha-<gitsha>`
  - semver release: `vX.Y.Z`

### 3.3 Images and Tags

Images (suggested):
- `m0club/engine:<tag>`
- `m0club/signer:<tag>`
- `m0club/api:<tag>`
- `m0club/indexer:<tag>`
- `m0club/realtime:<tag>`
- `m0club/jobs:<tag>`
- `m0club/dashboard:<tag>`

For local dev, images can be built from `infra/docker/images/*.Dockerfile`.

---

## 4. Configuration Model

### 4.1 Config Files

Config directory is environment-scoped:
- `config/dev.toml`
- `config/staging.toml`
- `config/prod.toml`

Domain catalogs:
- `config/markets/*.toml`

Risk + guardrails:
- `config/risk/*.toml`

Telemetry:
- `config/telemetry/*`

### 4.2 Environment Variables

All components accept a consistent baseline set:

- `M0_ENV` = `dev|staging|prod`
- `M0_CONFIG_PATH` = path to env config (mounted file)
- `M0_LOG_LEVEL` = `info|debug|warn|error`
- `M0_OTEL_EXPORTER_OTLP_ENDPOINT` (optional)
- `M0_PROMETHEUS_BIND` (optional)

Service-specific examples appear in the Appendix.

### 4.3 Secrets

Never store secrets in git.
Use one of:
- Kubernetes Secrets
- External Secrets Operator (recommended)
- Cloud Secret Manager injection

Secrets include:
- RPC credentials (if required)
- signer key material (or KMS references)
- DB credentials
- JWT signing keys / API keys
- third-party ingestion credentials

### 4.4 Recommended Secret Manager Patterns

Production recommended patterns:
- **KMS** for signer keys (no raw key material in pods)
- External secrets sync to K8s secrets at runtime
- short-lived tokens for DB access (if possible)

---

## 5. Local Deployment (Docker Compose)

### 5.1 Quickstart

```bash
cd infra/docker
docker compose -f compose.dev.yml up --build
```

To stop and remove volumes:
```bash
docker compose -f compose.dev.yml down -v
```

### 5.2 Local Solana Validator (Optional)

If you want to run programs locally:
- start local validator
- deploy Anchor programs to localnet
- point services to localnet RPC

Example:
```bash
./scripts/localnet.sh
./scripts/deploy_programs.sh localnet
```

### 5.3 Local Environment Variables

Common local overrides:
```bash
export M0_ENV=dev
export M0_API_BASE=http://localhost:8080
export M0_RPC_URL=http://localhost:8899
```

### 5.4 Validation

API health:
```bash
curl -s http://localhost:8080/health
```

Markets:
```bash
curl -s http://localhost:8080/markets | head
```

WebSocket:
```bash
# requires websocat installed
websocat ws://localhost:8090/ws
```

### 5.5 Troubleshooting Local

Common issues:
- ports in use (8080/8090/3000/5432/6379)
- docker network conflicts
- missing `.env` vars for services that require auth

Check logs:
```bash
docker compose -f compose.dev.yml logs -f api
docker compose -f compose.dev.yml logs -f engine
```

---

## 6. Staging Deployment (Kubernetes)

### 6.1 Cluster Baseline

Staging must provide:
- ingress controller (nginx/traefik/alb)
- cert manager (optional but recommended)
- metrics backend (Prometheus) and logs aggregator (Loki)
- persistent storage class for databases

### 6.2 Namespaces

Apply namespace manifests (if included):
```bash
kubectl apply -f infra/k8s/namespaces/
```

### 6.3 Ingress and TLS

Recommended:
- cert-manager with Let's Encrypt for staging
- separate subdomains:
  - `api.staging.m0club.com`
  - `dash.staging.m0club.com`

### 6.4 Datastores

Staging may use managed DBs or in-cluster charts.
Minimum required for most stacks:
- Postgres
- Redis

Optional:
- ClickHouse for analytics

### 6.5 Deploy via Helm

```bash
cd infra/k8s/helm
helm dependency update m0club
helm upgrade --install m0club ./m0club -f values.yaml --namespace m0club
```

### 6.6 Validation

```bash
kubectl -n m0club get pods
kubectl -n m0club get svc
kubectl -n m0club get ingress
```

Then:
```bash
curl -s https://api.staging.m0club.com/health
```

---

## 7. Production Deployment (Kubernetes)

### 7.1 Production Topology

A hardened production topology separates:
- ingestion workers
- model runtime workers
- signer agents (isolated node pool)
- public API
- realtime WS service
- indexers and jobs

### 7.2 Resource Sizing

Start conservative and scale with metrics:
- API: 2-4 replicas
- Realtime: 2-4 replicas
- Indexer: 1-2 replicas
- Engine: 2+ replicas by stage/role
- Signer: isolated, minimal replicas with strict access

### 7.3 Autoscaling

Use HPA based on:
- CPU for API/WS
- queue lag for ingestion
- publish latency for signer agents (custom metrics)

### 7.4 High Availability

- multi-AZ nodes
- pod disruption budgets for API/WS
- managed databases with HA enabled
- stateless services across replicas

### 7.5 Disaster Recovery

- daily DB backups, tested restore procedure
- store config and helm values in a secured repo
- store signer rotation and on-chain authority keys in a controlled process

---

## 8. On-Chain Program Deployment

### 8.1 Program Build

```bash
cd programs
anchor build
```

### 8.2 Program IDs and Registry

Ensure program IDs are stable:
- set IDs in `Anchor.toml`
- record in an internal registry file (recommended)

### 8.3 Deploy to Devnet/Testnet/Mainnet

```bash
anchor deploy --provider.cluster devnet
anchor deploy --provider.cluster mainnet
```

### 8.4 Upgrade Policy

Recommended:
- only upgrade using a timelock/controlled authority
- require staging soak period before mainnet
- publish release notes for state layout changes
- never change account layout without a migration plan

### 8.5 IDL Generation and Publishing

```bash
./scripts/gen_idl.sh
```

Commit generated IDLs under `programs/*/idl/` and ensure SDKs reference them.

### 8.6 Verifying On-Chain State

Use:
- Solana explorer / RPC calls
- indexer materialized views
- program-specific admin queries

---

## 9. Engine Deployment

### 9.1 Engine Roles

Typical roles:
- `ingest` (connectors, stream ingestion)
- `normalize` (canonicalization)
- `feature` (feature transforms)
- `model` (probability distributions)
- `bundle` (hashing + Merkle)
- `signer` (commit/reveal + tx submission)

### 9.2 Ingestion Connectors

Connectors can be enabled/disabled by config.
Rate limits and retries should be tuned per provider.

### 9.3 Feature Store Backends

Selectable backends (as scaffold):
- Postgres
- ClickHouse
- RocksDB (local edge)

### 9.4 Bundling and Hashing

Bundle format is deterministic and content-addressed.
Bundle hashing and optional Merkle proofs protect integrity and enable partial verification.

### 9.5 Signer Agent

Signer agents should:
- run in isolated node pools
- have restricted egress
- use KMS-backed signing or encrypted key mounts
- enforce replay protection windows

---

## 10. Services Deployment

### 10.1 API Gateway

Concerns:
- rate limiting
- auth (API keys, JWT)
- caching (Redis)
- pagination stability

### 10.2 Indexer

Concerns:
- RPC reliability
- reorg handling
- at-least-once event processing
- idempotent writes

### 10.3 Realtime WebSocket

Concerns:
- backpressure
- connection limits
- authentication and throttling
- fanout efficiency

### 10.4 Jobs

Concerns:
- scheduling
- idempotency
- backfill limits
- data retention

### 10.5 Dashboard

Concerns:
- CDN caching
- environment variables at build/runtime
- API base URL configuration
- security headers

---

## 11. Database and Migrations

### 11.1 Postgres

Recommended:
- managed Postgres
- logical backups enabled
- read replicas for heavy dashboard/query load

### 11.2 ClickHouse (Optional)

Use for:
- high-volume analytics
- time-series event materialization
- dashboard aggregate queries

### 11.3 Redis

Use for:
- API caching
- rate limiting
- short-lived realtime state

### 11.4 Migration Workflow

- version migrations
- apply migrations before rolling out new app versions
- verify schema version
- keep backward compatibility across one release window when possible

---

## 12. Observability

### 12.1 Metrics

Expose `/metrics` where applicable.
Track:
- ingestion throughput
- normalization failure rate
- bundle publish latency
- signer commit/reveal success rate
- API latency and error rates
- WS connection count

### 12.2 Logs

Use structured logs with:
- request id
- market id / epoch id (where relevant)
- component/role

### 12.3 Tracing (Optional)

OpenTelemetry endpoint can be configured via env vars.

### 12.4 SLOs and Alerts

Suggested SLOs:
- API availability >= 99.9%
- publish latency <= target window
- ingestion lag <= target window

Alerts:
- signer failures
- publish backlog
- RPC error spikes
- DB connection saturation
- websocket disconnect storm

---

## 13. Security Hardening

### 13.1 Network Policy

- deny all by default
- allow only required service-to-service traffic
- restrict signer pods egress

### 13.2 RBAC

- separate service accounts per component
- no cluster-admin service accounts for app pods

### 13.3 Image Signing and SBOM

Recommended:
- sign images with cosign
- generate SBOM and store artifacts in release pipeline

### 13.4 Dependency Scanning

- run cargo audit and npm audit in CI
- enable GitHub code scanning

### 13.5 Secrets Rotation

- schedule signer rotation
- rotate JWT secrets periodically
- rotate API keys and revoke compromised keys immediately

---

## 14. Release, Promotion, and Rollback

### 14.1 CI/CD Flow

Recommended flow:
1. merge to main triggers build + tests
2. build images tagged with SHA
3. staging deploy uses SHA tag
4. create release tag `vX.Y.Z`
5. production deploy uses semver tag

### 14.2 Staging Promotion

- run soak tests for at least 24 hours
- run k6 smoke and scenario tests
- validate signer commit/reveal in staging environment

### 14.3 Production Cutover

- deploy services first
- deploy engine next
- deploy signer agents last
- confirm on-chain program addresses match config

### 14.4 Rollback Procedures

Helm rollback:
```bash
helm -n m0club rollback m0club <REVISION>
```

Docker rollback (local):
```bash
docker compose -f compose.dev.yml down
git checkout <known-good-commit>
docker compose -f compose.dev.yml up --build
```

Signer rollback:
- if signer set rotation introduced issues, freeze rotation and revert to last known-good set (requires governance process)

---

## 15. Verification Checklist

- [ ] API `/health` returns ok
- [ ] `/markets` returns expected markets
- [ ] indexer is caught up (no growing lag)
- [ ] signer agent commits and reveals within epoch window
- [ ] finalization produces bundle hash and signatures
- [ ] replay protection prevents duplicate submissions
- [ ] dashboard loads and renders expected market data
- [ ] metrics scrape works
- [ ] alerts are configured and tested
- [ ] backups are enabled and restore was tested in staging

---

## 16. Troubleshooting

### 16.1 Common Failure Modes

- RPC provider rate limiting or downtime
- signer key misconfiguration / missing permissions
- DB schema mismatch (migrations not applied)
- websocket fanout overload (missing throttling)
- ingestion connector misbehaving (bad payloads)

### 16.2 Debug Commands

K8s:
```bash
kubectl -n m0club get pods
kubectl -n m0club describe pod <pod>
kubectl -n m0club logs -f <pod>
kubectl -n m0club exec -it <pod> -- sh
```

Helm:
```bash
helm -n m0club status m0club
helm -n m0club history m0club
```

---

## 17. Appendix

### 17.1 Example Environment Variable Set

```bash
export M0_ENV=staging
export M0_CONFIG_PATH=/etc/m0club/config/staging.toml
export M0_LOG_LEVEL=info

export M0_RPC_URL=https://api.devnet.solana.com
export M0_ORACLE_PROGRAM_ID=REPLACE_WITH_PROGRAM_ID
export M0_REGISTRY_PROGRAM_ID=REPLACE_WITH_PROGRAM_ID

export M0_PG_DSN=postgres://USER:PASS@HOST:5432/m0club
export M0_REDIS_URL=redis://HOST:6379/0
```

### 17.2 Example Helm Values Skeleton

```yaml
global:
  env: staging
  imageTag: "sha-REPLACE"
  configMapName: "m0club-config"
  secretName: "m0club-secrets"

api:
  replicas: 2
  resources:
    requests:
      cpu: "250m"
      memory: "512Mi"
    limits:
      cpu: "1000m"
      memory: "1Gi"

engine:
  replicas: 2

signer:
  replicas: 2
  nodeSelector:
    m0club.io/role: signer
  tolerations:
    - key: "m0club.io/signer"
      operator: "Equal"
      value: "true"
      effect: "NoSchedule"
```

### 17.3 Example Kubernetes Secret Manifests

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: m0club-secrets
  namespace: m0club
type: Opaque
stringData:
  M0_PG_DSN: "postgres://USER:PASS@HOST:5432/m0club"
  M0_REDIS_URL: "redis://HOST:6379/0"
  M0_JWT_SECRET: "REPLACE_WITH_STRONG_SECRET"
  M0_API_KEYS: "key1,key2,key3"
```
