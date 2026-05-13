#!/usr/bin/env zsh

set -euo pipefail
set -x

CONTAINER_NAME="newsletter-db"

# Stop + remove postgres container
docker rm -f "$CONTAINER_NAME" >/dev/null 2>&1 || true

# Stop Colima
colima stop

echo "Database + Colima stopped"
