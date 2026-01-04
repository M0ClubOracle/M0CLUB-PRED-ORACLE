
# Local Development Guide

This guide explains how to run M0Club locally with a Solana local validator, Anchor programs, and minimal engine/services.
It is designed to get a fully working developer loop on a single machine.

Repository: `m0club/`

---

## 0. Prerequisites

### 0.1 System requirements
- macOS, Linux, or Windows (WSL2 recommended)
- 8+ CPU cores recommended
- 16GB+ RAM recommended
- Docker Desktop (or Docker Engine) installed
- Node.js 18+
- Rust stable toolchain
- Git

### 0.2 Required tooling
- Solana CLI (matching your target cluster)
- Anchor CLI
- Yarn or pnpm (repo may use pnpm)

Install (example):
- Solana: follow Solana official install instructions
- Rust: https://rustup.rs
- Node: https://nodejs.org
- Anchor: `cargo install --git https://github.com/coral-xyz/anchor avm --locked`
  - `avm install latest`
  - `avm use latest`

Verify:
```bash
solana --version
anchor --version
rustc --version
node --version
docker --version
```

---

## 1. Repo Layout (Local Focus)

Local dev typically uses:
- `programs/` (Anchor programs)
- `core-engine/` (Rust engine services)
- `services/` (API gateway, realtime, backtest runner)
- `sdk/` (TypeScript/Rust/Python bindings)
- `infrastructure/` (docker compose, k8s manifests)
- `docs/` (this documentation)

---

## 2. Environment Setup

### 2.1 Clone and bootstrap
```bash
git clone <YOUR_REPO_URL> m0club
cd m0club
```

### 2.2 Install Node dependencies
If monorepo uses pnpm:
```bash
corepack enable
pnpm install
```

If yarn:
```bash
yarn install
```

### 2.3 Rust toolchain
Use stable:
```bash
rustup default stable
rustup update
```

Optional (faster builds):
```bash
rustup component add clippy rustfmt
```

---

## 3. Local Solana Cluster

### 3.1 Start local validator
Open a terminal and run:
```bash
solana-test-validator --reset
```

Keep it running.

### 3.2 Configure Solana CLI for localnet
In a second terminal:
```bash
solana config set --url http://127.0.0.1:8899
solana config get
```

### 3.3 Create a local keypair and airdrop
```bash
solana-keygen new --no-bip39-passphrase -o ~/.config/solana/id.json
solana airdrop 10
solana balance
```

---

## 4. Build and Deploy Programs (Anchor)

### 4.1 Enter programs workspace
```bash
cd programs
```

### 4.2 Install Anchor deps
```bash
pnpm install
```

If using yarn:
```bash
yarn
```

### 4.3 Build programs
```bash
anchor build
```

### 4.4 Deploy programs to localnet
```bash
anchor deploy
```

### 4.5 Run Anchor tests (optional)
```bash
anchor test
```

Notes:
- Tests require local validator running.
- If tests use program logs, ensure local validator is not rate-limited.

---

## 5. Configure Local Services

M0Club services use environment-based configs.
Local dev should use `.env.local` or `config/local.toml` (depending on implementation).

### 5.1 Create env file
From repo root:
```bash
cp .env.example .env.local
```

Edit `.env.local`:
```bash
M0_ENV=local
SOLANA_RPC_URL=http://127.0.0.1:8899
SOLANA_WS_URL=ws://127.0.0.1:8900

# Program IDs (after anchor deploy)
M0_ORACLE_PROGRAM_ID=<PASTE_FROM_ANCHOR_DEPLOY_OUTPUT>
M0_REGISTRY_PROGRAM_ID=<PASTE_FROM_ANCHOR_DEPLOY_OUTPUT>

# Signer agent (local mode)
M0_SIGNER_MODE=local
M0_SIGNER_KEYPAIR_PATH=./infrastructure/dev-keys/signer-1.json

# Storage
M0_POSTGRES_URL=postgres://m0:m0@127.0.0.1:5432/m0

# Event log
M0_EVENTLOG_BACKEND=file
M0_EVENTLOG_PATH=./.local/eventlog

# Feature store
M0_FEATURE_STORE_BACKEND=postgres
```

Do not commit `.env.local`.

---

## 6. Start Local Infra (Postgres + Optional NATS)

If the repo includes a local docker-compose, start it.

From repo root:
```bash
docker compose -f infrastructure/docker-compose.local.yml up -d
```

Verify Postgres:
```bash
docker ps
```

Optional: create DB schema
```bash
psql "postgres://m0:m0@127.0.0.1:5432/m0" -c "select 1;"
```

If migrations exist:
```bash
cd services
pnpm db:migrate
```

---

## 7. Run the Engine Locally

### 7.1 Build engine workspace
From repo root:
```bash
cd core-engine
cargo build
```

### 7.2 Run in local profile
The engine can run as a single process in local mode with embedded roles.

Example:
```bash
cargo run -p m0-engine --   --profile local   --rpc http://127.0.0.1:8899   --eventlog file://../.local/eventlog   --feature-store postgres://m0:m0@127.0.0.1:5432/m0   --signer local://../infrastructure/dev-keys/signer-1.json
```

If the repo uses config files:
```bash
cargo run -p m0-engine -- --config ../infrastructure/config/local.toml
```

Expected logs:
- connector started (fixture or mock)
- events ingested
- aggregation windows produced
- bundles hashed
- commits/reveals sent (if enabled)

---

## 8. Run the API Gateway

From repo root:
```bash
cd services/api-gateway
pnpm install
pnpm dev
```

Gateway should expose:
- health endpoint
- latest market outputs
- bundle metadata endpoints

Example:
```bash
curl http://127.0.0.1:8080/health
```

---

## 9. Run the Realtime Service (Optional)

From repo root:
```bash
cd services/realtime
pnpm install
pnpm dev
```

This service can provide:
- websocket subscriptions for latest outputs
- metrics for dashboards

---

## 10. Seed Local Markets

If the repo provides a market registry seeding script:

Example (TypeScript):
```bash
cd services/seed
pnpm seed:local
```

Or a CLI under `core-engine`:
```bash
cargo run -p m0-cli -- markets seed --cluster local
```

Seeding typically:
- creates registry accounts
- adds market definitions
- configures cadence and signer set id
- sets publish policies

---

## 11. Verify On-Chain State

Use Solana CLI:
```bash
solana program show <PROGRAM_ID>
```

If the repo has a CLI query:
```bash
cargo run -p m0-cli -- registry list --cluster local
cargo run -p m0-cli -- oracle latest --market NBA_LAL_BOS --cluster local
```

Use Anchor IDL client (if provided) to fetch accounts:
```bash
cd programs
anchor run fetch-registry
```

---

## 12. End-to-End Smoke Test

Minimal smoke test:
1) start validator
2) deploy programs
3) start infra (postgres)
4) run engine in fixture mode
5) confirm engine publishes a commit and reveal
6) query API gateway for latest output

Example checks:
```bash
curl http://127.0.0.1:8080/v1/markets
curl http://127.0.0.1:8080/v1/markets/NBA_LAL_BOS/latest
```

---

## 13. Troubleshooting

### 13.1 Anchor deploy fails
- ensure validator is running
- ensure you have SOL balance
- run `solana airdrop 10`

### 13.2 RPC timeouts
- local validator under load; reduce concurrency
- set `--commitment processed` for local dev

### 13.3 Missing signer threshold
- ensure signer keypair exists
- ensure signer agent is running (if separate)
- ensure registry signer set matches local keys

### 13.4 Postgres connection errors
- verify docker compose is up
- check port 5432 is free
- confirm M0_POSTGRES_URL

### 13.5 No outputs in API
- engine not running or not publishing
- markets not seeded
- engine running in a mode that does not submit commits/reveals

---

## 14. Local Security Notes

- never commit keys
- keep dev keys under `infrastructure/dev-keys/` and gitignore them if needed
- treat local signer keys as disposable

---

## 15. Quick Commands Summary

From repo root:

Start validator:
```bash
solana-test-validator --reset
```

Deploy programs:
```bash
cd programs
anchor build
anchor deploy
```

Start infra:
```bash
docker compose -f infrastructure/docker-compose.local.yml up -d
```

Run engine:
```bash
cd core-engine
cargo run -p m0-engine -- --profile local
```

Run API:
```bash
cd services/api-gateway
pnpm dev
```

---

## Links

- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx
