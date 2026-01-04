#!/usr/bin/env bash
set -euo pipefail

# Seed market definitions into the on-chain registry (or into the registry cache DB for local dev).
#
# Usage:
#   scripts/seed_markets.sh --env dev|staging|prod
#
# Environment variables:
#   M0_ENV             (default: dev)
#   M0_CONFIG_DIR      (default: ./config)
#
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ENVIRONMENT="${M0_ENV:-dev}"
CONFIG_DIR="${M0_CONFIG_DIR:-$ROOT_DIR/config}"

if [[ "${1:-}" == "--env" ]]; then
  ENVIRONMENT="${2:-$ENVIRONMENT}"
fi

log() { printf "[seed_markets] %s\n" "$*"; }

MARKETS_DIR="$CONFIG_DIR/markets"
if [[ ! -d "$MARKETS_DIR" ]]; then
  log "Markets directory not found: $MARKETS_DIR"
  exit 1
fi

# Prefer a Rust CLI if present; fall back to Node CLI.
if [[ -x "$ROOT_DIR/target/debug/m0-cli" ]]; then
  CLI="$ROOT_DIR/target/debug/m0-cli"
elif [[ -f "$ROOT_DIR/Cargo.toml" ]]; then
  CLI="cargo run -p m0-cli --"
else
  CLI=""
fi

if [[ -z "$CLI" ]]; then
  log "No m0-cli found. Implement or build your registry seeding CLI."
  log "Expected: cargo run -p m0-cli -- market import --file <toml>"
  exit 1
fi

for f in "$MARKETS_DIR"/*.toml; do
  log "Importing markets from: $(basename "$f")"
  # Conceptual command; align with your CLI implementation.
  $CLI market import --env "$ENVIRONMENT" --file "$f"
done

log "Market seeding complete."
