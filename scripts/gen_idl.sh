#!/usr/bin/env bash
set -euo pipefail

# Generate Anchor IDLs and copy them into sdk packages.
#
# Usage:
#   scripts/gen_idl.sh
#
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

log() { printf "[gen_idl] %s\n" "$*"; }

if [[ ! -d "programs" ]]; then
  log "No programs directory found."
  exit 1
fi

log "Building programs and generating IDLs"
(cd programs && anchor build)

IDL_DIR="$ROOT_DIR/programs/target/idl"
if [[ ! -d "$IDL_DIR" ]]; then
  log "IDL directory not found: $IDL_DIR"
  exit 1
fi

# Copy into SDK locations if present
if [[ -d "$ROOT_DIR/sdk/ts" ]]; then
  mkdir -p "$ROOT_DIR/sdk/ts/idl"
  cp -v "$IDL_DIR"/*.json "$ROOT_DIR/sdk/ts/idl/" || true
fi

if [[ -d "$ROOT_DIR/sdk/rust" ]]; then
  mkdir -p "$ROOT_DIR/sdk/rust/idl"
  cp -v "$IDL_DIR"/*.json "$ROOT_DIR/sdk/rust/idl/" || true
fi

if [[ -d "$ROOT_DIR/sdk/python" ]]; then
  mkdir -p "$ROOT_DIR/sdk/python/idl"
  cp -v "$IDL_DIR"/*.json "$ROOT_DIR/sdk/python/idl/" || true
fi

log "IDL generation complete."
