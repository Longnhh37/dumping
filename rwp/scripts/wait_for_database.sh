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

echo "postgres is ready"
