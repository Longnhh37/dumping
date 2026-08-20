#!/usr/bin/env zsh

set -euo pipefail
set -x

CONTAINER_NAME="newsletter-db"

DB_USER="${POSTGRES_USER:-postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:-password}"
DB_NAME="${POSTGRES_DB:-newsletter}"
DB_PORT="${POSTGRES_PORT:-5432}"

# Start Colima nếu chưa chạy
if ! colima status >/dev/null 2>&1; then
  colima start
fi

# Remove container cũ nếu tồn tại
docker rm -f "$CONTAINER_NAME" >/dev/null 2>&1 || true

# Start Postgres container
docker run \
  --name "$CONTAINER_NAME" \
  -e POSTGRES_USER="$DB_USER" \
  -e POSTGRES_PASSWORD="$DB_PASSWORD" \
  -e POSTGRES_DB="$DB_NAME" \
  -p "$DB_PORT":5432 \
  -d \
  postgres \
  postgres -N 1000

echo "Postgres started on port $DB_PORT"
