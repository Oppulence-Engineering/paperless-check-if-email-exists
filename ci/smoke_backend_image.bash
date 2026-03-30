#!/usr/bin/env bash

set -euo pipefail

IMAGE_TAG="${1:?usage: smoke_backend_image.bash <image-tag>}"
CONTAINER_NAME="reacher-backend-smoke-${RANDOM}-${RANDOM}"
HOST_PORT="${SMOKE_BACKEND_PORT:-18080}"
BASE_URL="http://127.0.0.1:${HOST_PORT}"

cleanup() {
  docker rm -f "${CONTAINER_NAME}" >/dev/null 2>&1 || true
}
trap cleanup EXIT

docker run -d \
  --name "${CONTAINER_NAME}" \
  -p "${HOST_PORT}:8080" \
  "${IMAGE_TAG}" >/dev/null

for _ in $(seq 1 60); do
  if curl --silent --fail "${BASE_URL}/healthz" >/dev/null; then
    break
  fi
  sleep 1
done

curl --silent --fail "${BASE_URL}/healthz" >/tmp/reacher-healthz.json
curl --silent --fail "${BASE_URL}/readyz" >/tmp/reacher-readyz.json
curl --silent --fail "${BASE_URL}/openapi.json" >/tmp/reacher-openapi.json

python3 - <<'PY'
import json
from pathlib import Path

healthz = json.loads(Path("/tmp/reacher-healthz.json").read_text())
assert healthz["status"] == "ok", healthz

readyz = json.loads(Path("/tmp/reacher-readyz.json").read_text())
assert readyz["status"] == "ok", readyz
assert readyz["checks"]["postgres"]["status"] == "not_configured", readyz
assert readyz["checks"]["rabbitmq"]["status"] == "not_configured", readyz

openapi = json.loads(Path("/tmp/reacher-openapi.json").read_text())
paths = openapi["paths"]
for required in ("/healthz", "/readyz", "/openapi.json", "/v1/check_email", "/v1/me"):
    assert required in paths, (required, sorted(paths.keys())[:10])
PY
