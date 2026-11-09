#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd -P)"

cd "$ROOT/frontend"
bun install
bun run build

cd "$ROOT"
cargo run -p ingress
