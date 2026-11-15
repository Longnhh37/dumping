#!/bin/zsh
set -euo pipefail

SCRIPTPATH="${0:A:h}"
cd "$SCRIPTPATH/.."

SERVICE="postgres"

cleanup() {
    docker compose down
}
trap cleanup EXIT

if ! docker compose up -d --wait; then
    echo "postgres failed to become healthy" >&2
    docker compose logs --tail=50 "$SERVICE" >&2
    exit 1
fi

if ! docker compose exec -T "$SERVICE" psql -U username -d to_do -tAc \
    "SELECT 1 FROM pg_database WHERE datname = 'auth'" | grep -q 1; then
    if ! docker compose exec -T "$SERVICE" createdb -U username auth; then
        echo "failed to create auth database" >&2
        docker compose logs --tail=50 "$SERVICE" >&2
        exit 1
    fi
fi

echo "to_do and auth databases are ready"
