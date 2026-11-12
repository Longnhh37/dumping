#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT"

cargo test --workspace

cd "$ROOT/frontend"
bun test
bun run typecheck
bun run build

if [[ "${RUN_POSTGRES_INTEGRATION:-0}" == "1" ]]; then
  "$ROOT/scripts/basic_test.sh"
fi
