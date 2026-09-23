#!/usr/bin/env bash

set -euo pipefail

IMAGE_TAG="${1:?usage: smoke_full_stack_image.bash <image-tag>}"
CONTAINER_NAME="reacher-full-stack-smoke-${RANDOM}-${RANDOM}"
RABBITMQ_NAME="reacher-full-stack-smoke-rabbitmq-${RANDOM}-${RANDOM}"
POSTGRES_NAME="reacher-full-stack-smoke-postgres-${RANDOM}-${RANDOM}"
NETWORK_NAME="reacher-full-stack-smoke-${RANDOM}-${RANDOM}"
HOST_PORT="${SMOKE_APP_PORT:-13080}"
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

echo "Starting full stack"
docker run -d \
  --name "${CONTAINER_NAME}" \
  --network "${NETWORK_NAME}" \
  -p "${HOST_PORT}:3000" \
  -e DATABASE_URL=postgres://reacher:reacher@postgres:5432/reacher \
  -e BETTER_AUTH_URL=https://smoke.example.test \
  -e BETTER_AUTH_SECRET=smoke-only-auth-secret-0123456789abcdef \
  -e SCIM_CREDENTIAL_HASH_SECRET=smoke-only-scim-secret-0123456789abcdef \
  -e BACKEND_JWT_AUDIENCE=check-if-email-exists-api \
  -e RESEND_API_KEY=smoke-not-used \
  -e RESEND_FROM=smoke@example.test \
  -e RCH__HEADER_SECRET=smoke-only-backend-secret \
  -e RCH__WORKER__ENABLE=true \
  -e RCH__WORKER__RABBITMQ__URL=amqp://guest:guest@rabbitmq:5672 \
  "${IMAGE_TAG}" >/dev/null

echo "Waiting for web and backend readiness"
for _ in $(seq 1 90); do
  if curl --silent --fail "${BASE_URL}/healthz" >/dev/null; then
    break
  fi
  sleep 1
done

curl --silent --fail "${BASE_URL}/healthz" | python3 -c 'import json,sys; assert json.load(sys.stdin)["status"] == "ok"'
for _ in $(seq 1 90); do
  if curl --silent --fail "${BASE_URL}/readyz" >/dev/null; then
    break
  fi
  sleep 1
done
curl --silent --fail "${BASE_URL}/readyz" | python3 -c 'import json,sys; assert json.load(sys.stdin)["status"] == "ready"'

status="$(curl --silent --output /tmp/reacher-api-response.json --write-out '%{http_code}' \
  --request POST "${BASE_URL}/v1/check_email" \
  --header 'Content-Type: application/json' \
  --header 'Authorization: Bearer rch_live_invalid' \
  --data '{"to_email":"test@valid.example.com","sandbox":true}')"
test "$status" = 401

status="$(curl --silent --output /dev/null --write-out '%{http_code}' \
  --request POST "${BASE_URL}/v1/inbound/providers/postmark/00000000-0000-0000-0000-000000000000/dummy")"
test "$status" = 404

status="$(curl --silent --output /dev/null --write-out '%{http_code}' \
  --request POST "${BASE_URL}/v1/check-email-with-onboard")"
test "$status" = 403
python3 -c 'from pathlib import Path; assert "API key authentication failed" in Path("/tmp/reacher-api-response.json").read_text()'

status="$(curl --silent --output /dev/null --write-out '%{http_code}' \
  --request POST "${BASE_URL}/v1/check_email" \
  --header 'Content-Type: application/json' \
  --header 'x-reacher-secret: smoke-only-backend-secret' \
  --data '{"to_email":"test@valid.example.com","sandbox":true}')"
test "$status" = 401
