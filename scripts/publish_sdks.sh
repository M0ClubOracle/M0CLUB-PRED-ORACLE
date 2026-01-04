#!/usr/bin/env bash
set -euo pipefail

# Publish SDKs to package registries.
#
# This script supports:
# - npm (TypeScript SDK)
# - PyPI (Python SDK)
#
# Usage:
#   scripts/publish_sdks.sh --dry-run
#
# Environment variables:
#   M0_VERSION          Version to publish (optional; will read from package files)
#   NPM_TOKEN           Auth for npm publish
#   PYPI_TOKEN          Auth for PyPI upload (twine)
#
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

DRY_RUN=0
if [[ "${1:-}" == "--dry-run" ]]; then
  DRY_RUN=1
fi

log() { printf "[publish_sdks] %s\n" "$*"; }

# --- TypeScript SDK
if [[ -d "sdk/ts" && -f "sdk/ts/package.json" ]]; then
  log "Publishing TypeScript SDK (sdk/ts)"
  pushd sdk/ts >/dev/null
  pnpm install --frozen-lockfile=false
  pnpm build || true

  if [[ "$DRY_RUN" == "1" ]]; then
    npm publish --dry-run
  else
    if [[ -z "${NPM_TOKEN:-}" ]]; then
      log "NPM_TOKEN not set; aborting npm publish."
      exit 1
    fi
    npm publish --access public
  fi
  popd >/dev/null
else
  log "No TypeScript SDK found at sdk/ts; skipping."
fi

# --- Python SDK
if [[ -d "sdk/python" && -f "sdk/python/pyproject.toml" ]]; then
  log "Publishing Python SDK (sdk/python)"
  pushd sdk/python >/dev/null
  python -m venv .venv_publish
  source .venv_publish/bin/activate
  pip install -U pip build twine
  python -m build

  if [[ "$DRY_RUN" == "1" ]]; then
    twine check dist/*
  else
    if [[ -z "${PYPI_TOKEN:-}" ]]; then
      log "PYPI_TOKEN not set; aborting PyPI upload."
      exit 1
    fi
    TWINE_USERNAME="__token__" TWINE_PASSWORD="$PYPI_TOKEN" twine upload dist/*
  fi
  deactivate || true
  popd >/dev/null
else
  log "No Python SDK found at sdk/python; skipping."
fi

log "SDK publish complete."
