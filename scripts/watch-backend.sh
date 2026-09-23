#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
stamp=$(mktemp)
backend_pid=

stop() {
  if [[ -n "$backend_pid" ]]; then
    kill "$backend_pid" 2>/dev/null || true
    wait "$backend_pid" 2>/dev/null || true
  fi
  rm -f "$stamp"
}
trap stop EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

start() {
  if cargo build --bin reacher_backend; then
    (cd backend && exec ../target/debug/reacher_backend) &
    backend_pid=$!
  else
    echo "Backend build failed; waiting for a source change." >&2
    backend_pid=
  fi
}

start
while true; do
  sleep 1
  if find backend core cli sqs -type f \( -name '*.rs' -o -name 'Cargo.toml' \) -newer "$stamp" -print -quit | grep -q . ||
    [[ Cargo.toml -nt "$stamp" || Cargo.lock -nt "$stamp" ]]; then
    if [[ -n "$backend_pid" ]]; then
      kill "$backend_pid" 2>/dev/null || true
      wait "$backend_pid" 2>/dev/null || true
    fi
    touch "$stamp"
    start
  fi
  if [[ -n "$backend_pid" ]] && ! kill -0 "$backend_pid" 2>/dev/null; then
    echo "Backend exited." >&2
    exit 1
  fi
done
