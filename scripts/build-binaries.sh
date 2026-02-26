#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
MODE="${1:-help}"

print_help() {
  cat <<'USAGE'
Mission Control binary build helper

Usage:
  ./scripts/build-binaries.sh preflight
  ./scripts/build-binaries.sh local

Commands:
  preflight  Build frontend and run Rust compile checks (no release binary output).
  local      Build a local release binary for the current machine in src-tauri/target/release/.

Examples:
  ./scripts/build-binaries.sh preflight
  ./scripts/build-binaries.sh local
USAGE
}

run_frontend_build() {
  pushd "$ROOT_DIR/frontend" >/dev/null
  npm ci
  npm run build
  popd >/dev/null
}

run_rust_compile_check() {
  pushd "$ROOT_DIR/src-tauri" >/dev/null
  cargo test --no-run
  popd >/dev/null
}

run_local_release_build() {
  pushd "$ROOT_DIR/src-tauri" >/dev/null
  cargo build --release
  popd >/dev/null
}

case "$MODE" in
  preflight)
    run_frontend_build
    run_rust_compile_check
    echo "✅ Preflight complete."
    ;;
  local)
    run_frontend_build
    run_local_release_build
    echo "✅ Local release build complete."
    echo "Binary output folder: src-tauri/target/release/"
    ;;
  help|-h|--help)
    print_help
    ;;
  *)
    echo "Unknown command: $MODE" >&2
    print_help
    exit 1
    ;;
esac
