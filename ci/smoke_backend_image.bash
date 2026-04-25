#!/usr/bin/env bash

set -euo pipefail

IMAGE_TAG="${1:?usage: smoke_backend_image.bash <image-tag>}"
CONTAINER_NAME="reacher-backend-smoke-${RANDOM}-${RANDOM}"
RABBITMQ_NAME="reacher-backend-smoke-rabbitmq-${RANDOM}-${RANDOM}"
POSTGRES_NAME="reacher-backend-smoke-postgres-${RANDOM}-${RANDOM}"
NETWORK_NAME="reacher-backend-smoke-${RANDOM}-${RANDOM}"
HOST_PORT="${SMOKE_BACKEND_PORT:-18080}"
BASE_URL="http://127.0.0.1:${HOST_PORT}"

dump_logs() {
  echo "::group::Smoke debug"
  docker ps -a || true
  for name in "${CONTAINER_NAME}" "${RABBITMQ_NAME}" "${POSTGRES_NAME}"; do
    if docker inspect "${name}" >/dev/null 2>&1; then
      echo "===== logs: ${name} ====="
      docker logs "${name}" || true
    fi
  done
  echo "::endgroup::"
}

cleanup() {
  docker rm -f "${CONTAINER_NAME}" >/dev/null 2>&1 || true
  docker rm -f "${RABBITMQ_NAME}" >/dev/null 2>&1 || true
  docker rm -f "${POSTGRES_NAME}" >/dev/null 2>&1 || true
  docker network rm "${NETWORK_NAME}" >/dev/null 2>&1 || true
}
trap 'status=$?; if [ "$status" -ne 0 ]; then dump_logs; fi; cleanup; exit "$status"' EXIT

docker network create "${NETWORK_NAME}" >/dev/null

echo "Starting RabbitMQ"
docker run -d \
  --name "${RABBITMQ_NAME}" \
  --network "${NETWORK_NAME}" \
  --network-alias rabbitmq \
  rabbitmq:3.8.22-management >/dev/null

echo "Starting Postgres"
docker run -d \
  --name "${POSTGRES_NAME}" \
  --network "${NETWORK_NAME}" \
  --network-alias postgres \
  -e POSTGRES_USER=reacher \
  -e POSTGRES_PASSWORD=reacher \
  -e POSTGRES_DB=reacher \
  postgres:16-alpine >/dev/null

for _ in $(seq 1 60); do
  if docker exec "${RABBITMQ_NAME}" rabbitmq-diagnostics -q check_running >/dev/null 2>&1; then
    break
  fi
  sleep 1
done

docker exec "${RABBITMQ_NAME}" rabbitmq-diagnostics -q check_running >/dev/null

for _ in $(seq 1 60); do
  if docker exec "${POSTGRES_NAME}" pg_isready -U reacher -d reacher >/dev/null 2>&1; then
    break
  fi
  sleep 1
done

docker exec "${POSTGRES_NAME}" pg_isready -U reacher -d reacher >/dev/null

echo "Starting backend"
docker run -d \
  --name "${CONTAINER_NAME}" \
  --network "${NETWORK_NAME}" \
  -p "${HOST_PORT}:8080" \
  -e RCH__HTTP_HOST=0.0.0.0 \
  -e RCH__WORKER__ENABLE=true \
  -e RCH__WORKER__RABBITMQ__URL=amqp://guest:guest@rabbitmq:5672 \
  -e RCH__STORAGE__POSTGRES__DB_URL=postgres://reacher:reacher@postgres:5432/reacher \
  "${IMAGE_TAG}" >/dev/null

echo "Waiting for backend health"
for _ in $(seq 1 60); do
  if curl --silent --fail "${BASE_URL}/healthz" >/dev/null; then
    break
  fi
  sleep 1
done

curl --silent --fail "${BASE_URL}/healthz" >/tmp/reacher-healthz.json || {
  docker logs "${CONTAINER_NAME}"
  exit 1
}
curl --silent --fail "${BASE_URL}/readyz" >/tmp/reacher-readyz.json || {
  docker logs "${CONTAINER_NAME}"
  exit 1
}
curl --silent --fail "${BASE_URL}/openapi.json" >/tmp/reacher-openapi.json || {
  docker logs "${CONTAINER_NAME}"
  exit 1
}

python3 - <<'PY'
import json
from pathlib import Path

healthz = json.loads(Path("/tmp/reacher-healthz.json").read_text())
assert healthz["status"] == "ok", healthz

readyz = json.loads(Path("/tmp/reacher-readyz.json").read_text())
assert readyz["status"] == "ok", readyz
assert readyz["checks"]["postgres"]["status"] == "ok", readyz
assert readyz["checks"]["rabbitmq"]["status"] == "ok", readyz

openapi = json.loads(Path("/tmp/reacher-openapi.json").read_text())
paths = openapi["paths"]
for required in ("/healthz", "/readyz", "/openapi.json", "/v1/check_email", "/v1/me"):
    assert required in paths, (required, sorted(paths.keys())[:10])
PY
