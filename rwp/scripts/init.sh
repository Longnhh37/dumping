#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT"

if [[ ! -f .env ]]; then
  cp .env.example .env
  echo "Created .env from .env.example"
fi

docker compose up -d --wait

if ! docker compose exec -T postgres psql -U username -d postgres -tAc \
  "SELECT 1 FROM pg_database WHERE datname = 'auth'" | grep -q 1; then
  docker compose exec -T postgres createdb -U username auth
fi

exec "$ROOT/ingress/scripts/run_server.sh"
