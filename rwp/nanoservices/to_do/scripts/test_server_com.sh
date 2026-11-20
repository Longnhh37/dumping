#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd -P)"
cd "$ROOT"

if [[ ! -f "$ROOT/.env" ]]; then
  echo "Missing $ROOT/.env; copy .env.example and configure it first." >&2
  exit 1
fi

# .env uses shell-compatible KEY=value lines and quotes for values with spaces.
set -a
source "$ROOT/.env"
set +a

for variable in TO_DO_DB_URL AUTH_DB_URL JWT_SECRET; do
  if [[ -z "${!variable:-}" ]]; then
    echo "Missing $variable in .env" >&2
    exit 1
  fi
done

for command in cargo docker xh jq; do
  if ! command -v "$command" >/dev/null 2>&1; then
    echo "Required command not found: $command" >&2
    exit 1
  fi
done

TODO_PORT="${TO_DO_WEB_PORT:-8080}"
AUTH_PORT="${AUTH_WEB_PORT:-8081}"
TODO_URL="http://127.0.0.1:${TODO_PORT}"
AUTH_URL="http://127.0.0.1:${AUTH_PORT}"
TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT/target}"
if [[ "$TARGET_DIR" != /* ]]; then
  TARGET_DIR="$ROOT/$TARGET_DIR"
fi
TODO_PID=""
AUTH_PID=""
TODO_LOG="$TARGET_DIR/to-do-test-server.log"
AUTH_LOG="$TARGET_DIR/auth-test-server.log"

cleanup() {
  local exit_code=$?

  if [[ -n "$TODO_PID" ]]; then
    kill "$TODO_PID" 2>/dev/null || true
    wait "$TODO_PID" 2>/dev/null || true
  fi
  if [[ -n "$AUTH_PID" ]]; then
    kill "$AUTH_PID" 2>/dev/null || true
    wait "$AUTH_PID" 2>/dev/null || true
  fi

  docker compose down >/dev/null 2>&1 || true

  if [[ "$exit_code" -ne 0 ]]; then
    echo "Test failed. Recent service logs:" >&2
    [[ ! -f "$TODO_LOG" ]] || tail -n 80 "$TODO_LOG" >&2
    [[ ! -f "$AUTH_LOG" ]] || tail -n 80 "$AUTH_LOG" >&2
  fi
  exit "$exit_code"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

wait_for_http() {
  local url="$1"
  local attempt
  for attempt in {1..60}; do
    if xh --ignore-stdin --no-check-status GET "$url" >/dev/null 2>&1; then
      return 0
    fi
    sleep 0.25
  done
  echo "Service did not respond at $url" >&2
  return 1
}

echo "Starting PostgreSQL..."
docker compose up -d --wait

if [[ "${AUTH_DB_URL##*/}" == "${TO_DO_DB_URL##*/}" ]]; then
  echo "Warning: AUTH_DB_URL and TO_DO_DB_URL use the same database in .env." >&2
fi

# The init SQL runs only on a new PostgreSQL data directory. Ensure auth exists
# when the named volume was created before that init script was added.
if ! docker compose exec -T postgres psql -U username -d postgres -tAc \
  "SELECT 1 FROM pg_database WHERE datname = 'auth'" | grep -q 1; then
  docker compose exec -T postgres createdb -U username auth
fi

echo "Building Auth and ToDo Axum services..."
cargo build -p to-do-axum-server --features auth-http --no-default-features
cargo build -p auth-axum-server

mkdir -p "$TARGET_DIR"
"$TARGET_DIR/debug/auth-axum-server" >"$AUTH_LOG" 2>&1 &
AUTH_PID=$!
# Both processes load settings from root .env.
"$TARGET_DIR/debug/to-do-axum-server" >"$TODO_LOG" 2>&1 &
TODO_PID=$!

wait_for_http "$AUTH_URL/api/v1/auth/login"
wait_for_http "$TODO_URL/api/v1/get"

RUN_ID="$(date +%s)-$$"
EMAIL="todo-test-${RUN_ID}@example.test"
PASSWORD="local-test-password-${RUN_ID}"
TITLE="todo-test-${RUN_ID}"

echo "Creating test account..."
xh --ignore-stdin --check-status POST "$AUTH_URL/api/v1/users/create" \
  email="$EMAIL" password="$PASSWORD" >/dev/null

TOKEN="$(xh --ignore-stdin --check-status -a "$EMAIL:$PASSWORD" \
  POST "$AUTH_URL/api/v1/auth/login" | jq -er '.')"

echo "Creating a ToDo item..."
CREATE_RESPONSE="$(xh --ignore-stdin --check-status --print=b \
  POST "$TODO_URL/api/v1/create" token:"$TOKEN" title="$TITLE" status=PENDING)"
ITEM_ID="$(jq -er --arg title "$TITLE" \
  '([.pending[], .done[]] | map(select(.title == $title)) | .[0].id)' \
  <<<"$CREATE_RESPONSE")"
echo "$CREATE_RESPONSE" | jq .

echo "Fetching item $ITEM_ID..."
FETCH_RESPONSE="$(xh --ignore-stdin --check-status --print=b \
  GET "$TODO_URL/api/v1/get/$ITEM_ID" token:"$TOKEN")"
jq -e --arg title "$TITLE" --argjson id "$ITEM_ID" \
  '.id == $id and .title == $title' <<<"$FETCH_RESPONSE" >/dev/null

echo "Updating item $ITEM_ID..."
UPDATE_RESPONSE="$(xh --ignore-stdin --check-status --print=b \
  PATCH "$TODO_URL/api/v1/update" token:"$TOKEN" \
  id:="$ITEM_ID" title="$TITLE" status=DONE)"
jq -e --arg title "$TITLE" --argjson id "$ITEM_ID" \
  'any(.done[]; .id == $id and .title == $title)' \
  <<<"$UPDATE_RESPONSE" >/dev/null

echo "Deleting item $ITEM_ID..."
DELETE_RESPONSE="$(xh --ignore-stdin --check-status --print=b \
  DELETE "$TODO_URL/api/v1/delete/$ITEM_ID" token:"$TOKEN")"
jq -e --argjson id "$ITEM_ID" \
  '([.pending[], .done[]] | all(.id != $id))' \
  <<<"$DELETE_RESPONSE" >/dev/null

echo "OK: account creation, login, and ToDo create/get/update/delete passed."
