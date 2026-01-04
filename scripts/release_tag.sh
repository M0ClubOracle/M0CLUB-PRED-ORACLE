#!/usr/bin/env bash
set -euo pipefail

# Create an annotated git tag for a release.
#
# Usage:
#   scripts/release_tag.sh v0.1.0
#
# Notes:
# - This script does not push automatically unless --push is provided.
# - It validates a clean working tree.
#
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

TAG="${1:-}"
PUSH=0
if [[ "${2:-}" == "--push" || "${1:-}" == "--push" ]]; then
  PUSH=1
fi

log() { printf "[release_tag] %s\n" "$*"; }

if [[ -z "$TAG" || "$TAG" == "--push" ]]; then
  log "Usage: scripts/release_tag.sh vX.Y.Z [--push]"
  exit 1
fi

if ! git diff --quiet || ! git diff --cached --quiet; then
  log "Working tree is not clean. Commit or stash changes before tagging."
  exit 1
fi

if git rev-parse "$TAG" >/dev/null 2>&1; then
  log "Tag already exists: $TAG"
  exit 1
fi

# Create a short changelog entry from commits since last tag
LAST_TAG="$(git describe --tags --abbrev=0 2>/dev/null || true)"
if [[ -n "$LAST_TAG" ]]; then
  RANGE="$LAST_TAG..HEAD"
else
  RANGE="HEAD"
fi

CHANGELOG_ENTRY="$(git log --no-merges --pretty=format:'- %s (%h)' $RANGE | head -n 50 || true)"

MSG="Release $TAG

Changes:
$CHANGELOG_ENTRY
"

git tag -a "$TAG" -m "$MSG"
log "Created tag: $TAG"

if [[ "$PUSH" == "1" ]]; then
  git push origin "$TAG"
  log "Pushed tag to origin: $TAG"
else
  log "Tag not pushed. Run: git push origin $TAG"
fi
