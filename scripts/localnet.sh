#!/usr/bin/env bash
set -euo pipefail

# Start a local Solana validator + optional services.
#
# Usage:
#   scripts/localnet.sh [--reset] [--with-docker]
#
# Environment variables:
#   SOLANA_LEDGER_DIR     Ledger directory (default: ./.solana-test-ledger)
#   SOLANA_RPC_PORT       RPC port (default: 8899)
#   SOLANA_WS_PORT        WS port (default: 8900)
#   M0_WITH_DOCKER        If "1", start docker compose for infra (postgres/redis/nats)
#
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

RESET=0
WITH_DOCKER="${M0_WITH_DOCKER:-0}"

for arg in "$@"; do
  case "$arg" in
    --reset) RESET=1 ;;
    --with-docker) WITH_DOCKER=1 ;;
    *) ;;
  esac
done

LEDGER_DIR="${SOLANA_LEDGER_DIR:-$ROOT_DIR/.solana-test-ledger}"
RPC_PORT="${SOLANA_RPC_PORT:-8899}"
WS_PORT="${SOLANA_WS_PORT:-8900}"

log() { printf "[localnet] %s\n" "$*"; }

if [[ "$WITH_DOCKER" == "1" ]]; then
  log "Starting local infra via docker compose"
  if [[ -f "infrastructure/docker-compose.local.yml" ]]; then
    docker compose -f infrastructure/docker-compose.local.yml up -d
  elif [[ -f "docker-compose.yml" ]]; then
    docker compose up -d
  else
    log "No docker compose file found, skipping."
  fi
fi

CMD=(solana-test-validator --rpc-port "$RPC_PORT" --ws-port "$WS_PORT" --ledger "$LEDGER_DIR")
if [[ "$RESET" == "1" ]]; then
  CMD+=(--reset)
fi

log "Starting solana-test-validator"
log "RPC: http://127.0.0.1:${RPC_PORT}"
log "WS:  ws://127.0.0.1:${WS_PORT}"
exec "${CMD[@]}"
