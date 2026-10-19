#!/bin/sh
set -eu

: "${DATABASE_URL:?DATABASE_URL is required}"
: "${RCH__HEADER_SECRET:?RCH__HEADER_SECRET is required to disable open backend access}"
: "${RCH__WORKER__RABBITMQ__URL:?RCH__WORKER__RABBITMQ__URL is required}"

export RCH__STORAGE__POSTGRES__DB_URL="${RCH__STORAGE__POSTGRES__DB_URL:-$DATABASE_URL}"
export RCH__AUTH__JWKS_URL="${RCH__AUTH__JWKS_URL:-http://127.0.0.1:3000/api/auth/jwks}"
export RCH__AUTH__ISSUER="${RCH__AUTH__ISSUER:-$BETTER_AUTH_URL}"
export RCH__AUTH__AUDIENCE="${RCH__AUTH__AUDIENCE:-$BACKEND_JWT_AUDIENCE}"

node /srv/web/scripts/migrate-auth.mjs
chromedriver --port=9515 &
driver_pid=$!
(cd /srv && unset PORT && exec /srv/reacher_backend) &
backend_pid=$!
node /srv/web/server.js &
web_pid=$!

stop() {
  kill "$web_pid" "$backend_pid" "$driver_pid" 2>/dev/null || true
  wait "$web_pid" "$backend_pid" "$driver_pid" 2>/dev/null || true
}
trap stop EXIT INT TERM

while kill -0 "$web_pid" "$backend_pid" "$driver_pid" 2>/dev/null; do
  sleep 1
done
exit 1
