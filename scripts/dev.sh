#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

export SQLX_OFFLINE=true
export DATABASE_URL="${DATABASE_URL:-postgres://postgres:postgres@127.0.0.1:25432/reacher}"
export BETTER_AUTH_URL="${BETTER_AUTH_URL:-http://localhost:3000}"
export BACKEND_API_URL="${BACKEND_API_URL:-http://127.0.0.1:8081}"
export BACKEND_JWT_AUDIENCE="${BACKEND_JWT_AUDIENCE:-check-if-email-exists-api}"
export BETTER_AUTH_SECRET="${BETTER_AUTH_SECRET:-local-better-auth-secret-for-development-only}"
export SCIM_CREDENTIAL_HASH_SECRET="${SCIM_CREDENTIAL_HASH_SECRET:-local-scim-secret-for-development-only}"
export RESEND_API_KEY="${RESEND_API_KEY:-local-development}"
export RESEND_FROM="${RESEND_FROM:-Check If Email Exists <noreply@example.com>}"
export AUTH_E2E_MODE="${AUTH_E2E_MODE:-1}"
export AUTH_E2E_OTP="${AUTH_E2E_OTP:-123456}"
export TRUSTED_PUBLIC_ORIGINS="${TRUSTED_PUBLIC_ORIGINS:-http://localhost:3000}"

export RCH__HTTP_HOST="${RCH__HTTP_HOST:-127.0.0.1}"
export RCH__HTTP_PORT="${RCH__HTTP_PORT:-8081}"
export RCH__HEADER_SECRET="${RCH__HEADER_SECRET:-local-backend-secret-for-development-only}"
export RCH__STORAGE__POSTGRES__DB_URL="${RCH__STORAGE__POSTGRES__DB_URL:-$DATABASE_URL}"
export RCH__WORKER__ENABLE="${RCH__WORKER__ENABLE:-true}"
export RCH__WORKER__RABBITMQ__URL="${RCH__WORKER__RABBITMQ__URL:-amqp://guest:guest@127.0.0.1:35672}"
export RCH__AUTH__JWKS_URL="${RCH__AUTH__JWKS_URL:-http://127.0.0.1:3000/api/auth/jwks}"
export RCH__AUTH__ISSUER="${RCH__AUTH__ISSUER:-$BETTER_AUTH_URL}"
export RCH__AUTH__AUDIENCE="${RCH__AUTH__AUDIENCE:-$BACKEND_JWT_AUDIENCE}"

docker compose up -d --wait postgres rabbitmq
if [[ ! -d web/node_modules ]]; then
  (cd web && pnpm install --frozen-lockfile)
fi
(cd web && pnpm auth:migrate)

echo "Backend: http://127.0.0.1:${RCH__HTTP_PORT}"
echo "Web:     ${BETTER_AUTH_URL}"
bash scripts/watch-backend.sh &
backend_pid=$!
(cd web && pnpm exec next dev --port 3000) &
web_pid=$!

cleanup() {
  kill "$backend_pid" "$web_pid" 2>/dev/null || true
  wait "$backend_pid" "$web_pid" 2>/dev/null || true
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
wait -n "$backend_pid" "$web_pid"
