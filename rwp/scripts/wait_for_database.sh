#!/bin/zsh
set -euo pipefail

SCRIPTPATH="${0:A:h}"
cd "$SCRIPTPATH/.."

SERVICE="to-do-postgres"
PG_USER="username"
MAX_TRIES=30

docker compose up -d

tries=0
until docker compose exec -T "$SERVICE" pg_isready -U "$PG_USER"; do
    tries=$((tries + 1))
    if (( tries >= MAX_TRIES )); then
        echo "postgres did not become ready in time" >&2
        docker compose logs --tail=50 "$SERVICE" >&2
        exit 1
    fi
    echo "waiting for postgres ($tries/$MAX_TRIES)"
    sleep 2
done

echo "postgres is ready"

docker compose down
