#!/usr/bin/env zsh

set -euo pipefail

./scripts/init_db.sh

export DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter

until pg_isready -h localhost -p 5432 -U postgres; do
    echo "waiting for progres..."
    sleep 1
done

echo "posgres is ready.\n"

sqlx migrate run

echo "init done"
