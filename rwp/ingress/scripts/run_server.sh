#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd -P)"

cd "$ROOT/frontend"
bun install
bun run build

cd "$ROOT"
export $(cat .env | xargs)
cargo run -p ingress
