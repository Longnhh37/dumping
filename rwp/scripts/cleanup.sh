#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT"

docker compose down
echo "Stopped the local PostgreSQL container. Database data was preserved."
