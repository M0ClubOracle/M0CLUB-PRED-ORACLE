#!/usr/bin/env bash
set -euo pipefail

# Build everything in the monorepo:
# - Anchor programs
# - Rust workspace crates
# - Node services (if present)

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

log() { printf "[build_all] %s\n" "$*"; }

log "Building Rust workspace"
if [[ -f "Cargo.toml" ]]; then
  cargo build --workspace --all-targets
else
  log "No Cargo.toml at repo root, skipping Rust workspace build."
fi

if [[ -d "programs" ]]; then
  log "Building Anchor programs"
  (cd programs && anchor build)
else
  log "No programs/ directory, skipping Anchor build."
fi

if [[ -f "pnpm-workspace.yaml" || -f "package.json" ]]; then
  log "Building Node packages"
  pnpm -r build || pnpm build || true
else
  log "No Node workspace detected, skipping Node build."
fi

log "Build complete."
