#!/usr/bin/env bash
set -euo pipefail

# Deploy Anchor programs to the configured Solana cluster.
#
# Usage:
#   scripts/deploy_programs.sh --cluster localnet|devnet|mainnet-beta
#
# Environment variables:
#   ANCHOR_PROVIDER_URL
#   ANCHOR_WALLET
#
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

CLUSTER="${1:-}"
if [[ "$CLUSTER" == "--cluster" ]]; then
  CLUSTER="${2:-}"
fi

if [[ -z "$CLUSTER" ]]; then
  CLUSTER="${SOLANA_CLUSTER:-localnet}"
fi

log() { printf "[deploy_programs] %s\n" "$*"; }

if [[ ! -d "programs" ]]; then
  log "No programs directory found."
  exit 1
fi

case "$CLUSTER" in
  localnet)
    export ANCHOR_PROVIDER_URL="${ANCHOR_PROVIDER_URL:-http://127.0.0.1:8899}"
    ;;
  devnet)
    export ANCHOR_PROVIDER_URL="${ANCHOR_PROVIDER_URL:-https://api.devnet.solana.com}"
    ;;
  mainnet-beta)
    export ANCHOR_PROVIDER_URL="${ANCHOR_PROVIDER_URL:-https://api.mainnet-beta.solana.com}"
    ;;
  *)
    log "Unknown cluster: $CLUSTER"
    exit 1
    ;;
esac

export ANCHOR_WALLET="${ANCHOR_WALLET:-$HOME/.config/solana/id.json}"

log "Deploying to $CLUSTER"
log "ANCHOR_PROVIDER_URL=$ANCHOR_PROVIDER_URL"
log "ANCHOR_WALLET=$ANCHOR_WALLET"

(cd programs && anchor build && anchor deploy)

log "Programs deployed."
