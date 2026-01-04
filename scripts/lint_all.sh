#!/usr/bin/env bash
set -euo pipefail

# Lint everything:
# - rustfmt + clippy
# - Anchor (build)
# - Node lint (eslint) if available

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

log() { printf "[lint_all] %s\n" "$*"; }

if [[ -f "Cargo.toml" ]]; then
  log "Running rustfmt (check)"
  cargo fmt --all -- --check

  log "Running clippy"
  cargo clippy --workspace --all-targets -- -D warnings
fi

if [[ -d "programs" ]]; then
  log "Anchor build (sanity)"
  (cd programs && anchor build)
fi

if [[ -f "pnpm-workspace.yaml" || -f "package.json" ]]; then
  log "Node lint"
  pnpm -r lint || pnpm lint || true
fi

log "Lint complete."
