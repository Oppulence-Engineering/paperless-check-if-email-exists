#!/usr/bin/env bash
# Regenerate Orval clients/mocks from the selected backend OpenAPI and verify the tree is clean.
set -euo pipefail

cd "$(dirname "$0")/.."
OPENAPI="${BACKEND_OPENAPI_PATH:-config/contracts/backend.openapi.json}"

echo "==> contracts:generate"
pnpm contracts:generate

echo "==> contracts:check"
pnpm contracts:check

if command -v git >/dev/null 2>&1; then
  if git diff --quiet -- lib/api/generated 2>/dev/null; then
    echo "==> generated clients match OpenAPI"
  else
    echo "==> OpenAPI drift detected in lib/api/generated (review diff)"
    git diff --stat -- lib/api/generated || true
  fi
fi

echo "==> source: $OPENAPI"
