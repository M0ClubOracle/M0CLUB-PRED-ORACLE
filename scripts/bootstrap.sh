#!/usr/bin/env bash
set -euo pipefail

# M0Club bootstrap script
# - installs or validates required toolchains
# - sets up node dependencies
# - prepares local folders and pre-commit hooks (optional)
#
# This script is designed to be safe to re-run.
#
# Supported platforms: Linux, macOS
#
# Environment variables:
#   M0_SKIP_SOLANA=1       Skip Solana CLI checks/install hints
#   M0_SKIP_ANCHOR=1       Skip Anchor checks/install hints
#   M0_SKIP_NODE=1         Skip Node/PNPM checks/install hints
#   M0_SKIP_RUST=1         Skip Rust checks/install hints
#   M0_ENABLE_GITHOOKS=1   Enable git hooks via pre-commit (if present)
#

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

log()  { printf "[bootstrap] %s\n" "$*"; }
warn() { printf "[bootstrap][warn] %s\n" "$*" >&2; }
die()  { printf "[bootstrap][error] %s\n" "$*" >&2; exit 1; }

need_cmd() {
  command -v "$1" >/dev/null 2>&1
}

check_version() {
  local cmd="$1"
  local want="$2"
  local got="$($cmd 2>/dev/null || true)"
  log "$cmd => ${got:-unknown}"
  if [[ -n "$want" && -n "$got" ]]; then
    true
  fi
}

log "Repo: $ROOT_DIR"

# --- Rust toolchain
if [[ "${M0_SKIP_RUST:-0}" != "1" ]]; then
  if need_cmd rustc && need_cmd cargo; then
    check_version "rustc --version" ""
    check_version "cargo --version" ""
  else
    warn "Rust not found. Install via rustup: https://rustup.rs/"
    die "Missing Rust toolchain (rustc/cargo)."
  fi

  # Optional components
  rustup component add rustfmt >/dev/null 2>&1 || true
  rustup component add clippy  >/dev/null 2>&1 || true
fi

# --- Node / PNPM
if [[ "${M0_SKIP_NODE:-0}" != "1" ]]; then
  if need_cmd node; then
    check_version "node --version" ""
  else
    warn "Node.js not found. Install Node.js 18+."
    die "Missing Node.js."
  fi

  if need_cmd pnpm; then
    check_version "pnpm --version" ""
  else
    warn "pnpm not found. Install: npm i -g pnpm"
    die "Missing pnpm."
  fi

  # Install Node dependencies if workspace present
  if [[ -f "pnpm-workspace.yaml" || -f "package.json" ]]; then
    log "Installing Node dependencies (pnpm install)"
    pnpm install --frozen-lockfile=false
  else
    warn "No Node workspace found (pnpm-workspace.yaml/package.json missing). Skipping pnpm install."
  fi
fi

# --- Solana CLI
if [[ "${M0_SKIP_SOLANA:-0}" != "1" ]]; then
  if need_cmd solana; then
    check_version "solana --version" ""
  else
    warn "Solana CLI not found."
    warn "Install: https://docs.solana.com/cli/install-solana-cli-tools"
    die "Missing Solana CLI."
  fi
fi

# --- Anchor
if [[ "${M0_SKIP_ANCHOR:-0}" != "1" ]]; then
  if need_cmd anchor; then
    check_version "anchor --version" ""
  else
    warn "Anchor not found."
    warn "Install via avm: https://www.anchor-lang.com/docs/avm"
    die "Missing Anchor."
  fi
fi

# --- Local folders
mkdir -p .m0data artifacts/calibration/{dev,staging,prod} || true
mkdir -p target || true

# --- Git hooks (optional)
if [[ "${M0_ENABLE_GITHOOKS:-0}" == "1" ]]; then
  if need_cmd pre-commit; then
    log "Installing pre-commit hooks"
    pre-commit install
  else
    warn "pre-commit not found; skipping git hooks. Install: pipx install pre-commit"
  fi
fi

log "Bootstrap complete."
