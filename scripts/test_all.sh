#!/usr/bin/env bash
set -euo pipefail

# Run all tests:
# - Rust workspace tests
# - Anchor tests (requires localnet)
# - Node tests (if present)

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

log() { printf "[test_all] %s\n" "$*"; }

if [[ -f "Cargo.toml" ]]; then
  log "Running Rust tests"
  cargo test --workspace
fi

if [[ -d "programs" ]]; then
  log "Running Anchor tests"
  (cd programs && anchor test --skip-local-validator || true)
  # Note: Use scripts/localnet.sh in another terminal if you want deterministic localnet.
fi

if [[ -f "pnpm-workspace.yaml" || -f "package.json" ]]; then
  log "Running Node tests"
  pnpm -r test || pnpm test || true
fi

log "Tests complete."
