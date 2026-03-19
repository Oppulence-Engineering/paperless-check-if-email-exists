#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

ensure_cargo() {
  if command -v cargo >/dev/null 2>&1; then
    return
  fi

  if [ -x "${HOME}/.cargo/bin/cargo" ]; then
    export PATH="${HOME}/.cargo/bin:${PATH}"
    return
  fi

  local toolchain_cargo
  toolchain_cargo="$(find "${HOME}/.rustup/toolchains" -path '*/bin/cargo' -type f 2>/dev/null | head -n 1 || true)"
  if [ -n "${toolchain_cargo}" ]; then
    export PATH="$(dirname "${toolchain_cargo}"):${PATH}"
    return
  fi

  echo "sdk-autogen: cargo not found on PATH" >&2
  exit 1
}

should_regenerate() {
  if [ "${FORCE_SDK_AUTOGEN:-0}" = "1" ]; then
    return 0
  fi

  local staged_files
  staged_files="$(git diff --cached --name-only --diff-filter=ACMR)"
  if [ -z "${staged_files}" ]; then
    return 1
  fi

  while IFS= read -r file; do
    case "${file}" in
      backend/src/*|backend/Cargo.toml|backend/openapi.json|Makefile)
        return 0
        ;;
    esac
  done <<< "${staged_files}"

  return 1
}

ensure_cargo

if ! should_regenerate; then
  exit 0
fi

echo "sdk-autogen: regenerating backend OpenAPI and SDKs"
cargo run -p reacher_backend --bin generate_openapi
make sdk-generate-all

git add backend/openapi.json sdks/golang sdks/typescript/src
