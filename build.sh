#!/usr/bin/env bash
set -euo pipefail

readonly TARGET="wasm32-wasip2"
readonly PROFILE="release"
readonly BIN_NAME="country_lookup"
readonly INPUT="target/${TARGET}/${PROFILE}/${BIN_NAME}.wasm"
readonly OUTPUT="target/${TARGET}/${PROFILE}/${BIN_NAME}_init.wasm"

cd "$(dirname "$0")"

echo "==> Compiling WebAssembly component (${TARGET})..."
cargo build --target "${TARGET}" --profile "${PROFILE}"

echo "==> Running Wizer pre-initialization..."
wasmtime wizer \
  --keep-init-func=true \
  -W unknown-imports-trap \
  -o "${OUTPUT}" \
  "${INPUT}"

echo "==> Pre-initialization complete: ${OUTPUT}"
