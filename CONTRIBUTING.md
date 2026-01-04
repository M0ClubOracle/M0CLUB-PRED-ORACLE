
# Contributing to M0Club

Thanks for your interest in contributing to M0Club. This guide explains how to set up the repo, make changes,
run tests, and submit a high-quality pull request.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Repository Layout](#repository-layout)
- [Development Setup](#development-setup)
- [Local Workflow](#local-workflow)
- [Code Style](#code-style)
- [Testing](#testing)
- [Security](#security)
- [Release Process](#release-process)
- [Conventional Commits](#conventional-commits)
- [PR Checklist](#pr-checklist)
- [Getting Help](#getting-help)

## Code of Conduct

By participating, you are expected to uphold the [Code of Conduct](CODE_OF_CONDUCT.md).

## Repository Layout

M0Club is a monorepo. The key directories are:

- `programs/` — Solana Anchor programs (on-chain)
- `core-engine/` — high-performance modeling engine (off-chain)
- `services/` — API Gateway, realtime, indexer, dashboard, jobs
- `sdk/` — TS/Rust/Python SDKs + shared type definitions
- `infra/` — docker-compose, Kubernetes manifests/helm, terraform scaffolds, monitoring
- `tests/` — integration/load/fuzz suites
- `docs/` — architecture, specs, ops, and guides
- `scripts/` — dev tooling for builds/tests/deploys

## Development Setup

### Prerequisites

- Git
- Rust (pinned by `rust-toolchain.toml`)
- Node.js 20+ (dashboard + TS SDK/tests)
- Docker + Docker Compose (recommended for local services)
- Python 3.10+ (Python SDK, fuzz harness)
- Solana + Anchor (only required for building/deploying on-chain programs)

### Install Rust toolchain

This repo pins a stable Rust toolchain for off-chain components:

```bash
rustup show
rustc --version
cargo --version
```

If you do not have the pinned toolchain installed:

```bash
rustup toolchain install 1.78.0 --profile minimal --component rustfmt --component clippy --component rust-src --component llvm-tools-preview
```

### Install Node dependencies

```bash
cd services/dashboard
npm install
```

TypeScript SDK:

```bash
cd sdk/ts
npm install
```

Tests:

```bash
cd tests
npm install
```

### Optional: Install Solana + Anchor

Follow the official instructions for Solana + Anchor. Their toolchains are versioned separately from this repo.

## Local Workflow

### Start local environment

The quickest path is docker-compose:

```bash
cd infra/docker
docker compose -f compose.dev.yml up --build
```

This starts:
- postgres + redis
- api-gateway on http://localhost:8080
- realtime on http://localhost:8090
- indexer and jobs workers
- dashboard on http://localhost:3000

### Build all (host)

If your repo includes a `scripts/build_all.sh`, run:

```bash
./scripts/build_all.sh
```

Otherwise, build components manually:

Engine:

```bash
cd core-engine
cargo build
```

Services:

```bash
cd services
cargo build
```

Rust SDK:

```bash
cd sdk/rust
cargo build
```

### Lint and format

Rust (engine/services/sdk):

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

TypeScript (SDK/tests/dashboard): follow each package's scripts.

### Run tests

Integration tests require the gateway running (default: http://localhost:8080).

```bash
cd tests
npm test
```

Load tests (requires k6 installed):

```bash
cd tests
npm run k6:smoke
```

Python fuzz (no external deps beyond python):

```bash
cd tests
npm run fuzz:engine
```

## Code Style

### Rust

- Prefer explicit error types for library crates.
- Use `anyhow` only at binary boundaries.
- Avoid panics in production code paths.
- Keep modules small and focused.
- Use `tracing` for structured logs.

### TypeScript

- Keep public APIs in `sdk/ts/src/index.ts`.
- Ensure types remain compatible with `sdk/types/*`.
- Avoid breaking changes without MAJOR version bump.

### Docs

- Keep specs precise and implementation-independent where possible.
- Update docs with any protocol or API change.

## Testing

### What to test

- programs: instruction validation, state transitions, replay protection, signer rotation
- engine: pipeline correctness, deterministic outputs, invariants
- services: API stability, pagination, auth, rate limits
- sdk: client compatibility with gateway responses

### Adding tests

- Add integration tests to `tests/integration/`
- Add k6 scenarios under `tests/load/`
- Add fuzz scaffolds under `tests/fuzz/`

## Security

### Reporting vulnerabilities

Do not open public issues for security vulnerabilities.

Email:
- security@m0club.com

Include:
- summary of impact
- reproduction steps
- affected versions/commits
- suggested remediation (if known)

### Secrets management

- Never commit credentials, private keys, or seed phrases.
- Use `.env` locally and Kubernetes Secrets for clusters.
- For signers, prefer managed KMS or HSM-backed keys when possible.

## Release Process

Releases are handled via CI workflows and tags.

Typical flow:
1. Update `CHANGELOG.md`
2. Bump versions (sdk packages, docker images, chart version if needed)
3. Create a release tag:
   ```bash
   ./scripts/release_tag.sh v0.1.1
   ```
4. CI builds and publishes artifacts (images + SDK packages).

## Conventional Commits

We recommend conventional commits to improve changelogs and release automation:

- `feat:` new feature
- `fix:` bug fix
- `docs:` documentation
- `chore:` tooling/maintenance
- `refactor:` refactor without behavior change
- `test:` tests

Examples:
- `feat(engine): add bayesian calibration stage`
- `fix(api): ensure request id propagated to logs`
- `docs(protocol): clarify oracle output format`

## PR Checklist

Before opening a PR, ensure:

- [ ] Code builds locally
- [ ] Unit/integration tests pass
- [ ] Lint/format pass
- [ ] Docs updated (if behavior changed)
- [ ] No secrets committed
- [ ] Breaking changes clearly marked
- [ ] Changelog updated (if user-visible)

## Getting Help

- Open an issue with reproduction steps and logs.
- Include your platform info and versions.
- For sensitive items, email security@m0club.com.

Links:
- Website: https://m0club.com/
- X: https://x.com/M0Clubonx
