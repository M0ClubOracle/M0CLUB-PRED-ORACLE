#!/usr/bin/env bash
set -euo pipefail

# Smoke test for M0Club stack.
# Checks:
# - API health
# - markets list endpoint
# - latest endpoint for one market
# - optional verification status flag presence
#
# Environment variables:
#   M0_API_BASE (default: http://127.0.0.1:8080)
#   M0_MARKET_ID (default: NBA_LAL_BOS)
#
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

API_BASE="${M0_API_BASE:-http://127.0.0.1:8080}"
MARKET_ID="${M0_MARKET_ID:-NBA_LAL_BOS}"

log() { printf "[smoke_test] %s\n" "$*"; }

need() {
  command -v "$1" >/dev/null 2>&1 || { log "Missing dependency: $1"; exit 1; }
}

need curl
need jq

log "API_BASE=$API_BASE"
log "Checking /healthz"
curl -fsS "$API_BASE/healthz" | jq . >/dev/null

log "Listing markets"
curl -fsS "$API_BASE/v1/markets" | jq 'length' >/dev/null

log "Fetching latest for $MARKET_ID"
curl -fsS "$API_BASE/v1/markets/$MARKET_ID/latest" | jq '.bundle_content_hash' >/dev/null

log "Smoke test passed."
