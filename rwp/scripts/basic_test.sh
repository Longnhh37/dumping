#!/usr/bin/env zsh
# File: scripts/basic_test.sh
# Smoke test for the PostgreSQL-backed axum to-do server.
# Requires: cargo, xh, jq.

set -euo pipefail

# Run everything from the workspace root (parent of scripts/)
cd "${0:A:h}/.."

OUTPUT="$(mktemp)"
BIN="${CARGO_TARGET_DIR:-target}/release/to-do-axum-server"
BASE=":8080/api/v1"
TOKEN="${TOKEN:?export TOKEN=<jwt> first, e.g. TOKEN=\$(xh --ignore-stdin -a EMAIL:PASS POST :8001/api/v1/auth/login | jq -r .)}"
RUN_ID="${$}-$(date +%s)"
WRITING="smoke-writing-${RUN_ID}"
CODING="smoke-coding-${RUN_ID}"
WRITING_ID=""
CODING_ID=""

# Build synchronously first, so $! below is the PID of the server itself
echo "Building axum server..."
cargo build --release -p to-do-axum-server

# Always stop the server, even when a step fails
PID=""
cleanup() {
  if [[ -n "$PID" ]]; then
    [[ -z "$WRITING_ID" ]] || xh --ignore-stdin DELETE "$BASE/delete/$WRITING_ID" token:"$TOKEN" &>/dev/null || true
    [[ -z "$CODING_ID" ]] || xh --ignore-stdin DELETE "$BASE/delete/$CODING_ID" token:"$TOKEN" &>/dev/null || true
    kill "$PID" 2>/dev/null || true
    wait "$PID" 2>/dev/null || true
  fi
  rm -f "$OUTPUT"
}
trap cleanup EXIT

"$BIN" &
PID=$!
echo "Server started with PID: $PID"

# Wait until the server answers instead of a blind sleep
for _ in {1..30}; do
  xh --ignore-stdin --check-status GET "$BASE/get" token:"$TOKEN" &>/dev/null && break
  sleep 0.2
done

# Run one request, log its JSON body to $OUTPUT, fail on non-2xx
step() {
  local label=$1
  shift
  echo "$label"
  echo "### $label" >>"$OUTPUT"
  xh --ignore-stdin --check-status "$@" token:"$TOKEN" | jq . >>"$OUTPUT"
}

create_item() {
  local title=$1 response
  response=$(xh --ignore-stdin --check-status POST "$BASE/create" title="$title" status=PENDING token:"$TOKEN")
  jq . <<<"$response" >>"$OUTPUT"
  jq -er --arg title "$title" '([.pending[], .done[]] | map(select(.title == $title)) | last | .id)' <<<"$response"
}

echo "create writing"
echo "### create writing" >>"$OUTPUT"
WRITING_ID=$(create_item "$WRITING")
echo "create coding"
echo "### create coding" >>"$OUTPUT"
CODING_ID=$(create_item "$CODING")
step "get all" GET "$BASE/get"
step "get by id" GET "$BASE/get/$WRITING_ID"
step "delete coding" DELETE "$BASE/delete/$CODING_ID"

# Update uses the numeric item ID and requires the `token` header.
step "update writing" PATCH "$BASE/update" id:="$WRITING_ID" title="$WRITING" status=DONE

# update without a token must be rejected with 401
echo "update without token (expect 401)"
if xh --ignore-stdin --check-status PATCH "$BASE/update" id:="$WRITING_ID" title="$WRITING" status=DONE &>/dev/null; then
  echo "FAIL: update without token was accepted" >&2
  exit 1
fi

# Final state: "writing" is done, nothing is pending
xh --ignore-stdin GET "$BASE/get" token:"$TOKEN" |
  jq -e --arg writing "$WRITING" \
    'any(.done[]; .title == $writing and .status == "DONE") and all(.pending[]; .title != $writing)' >/dev/null ||
  {
    echo "FAIL: unexpected final state" >&2
    exit 1
  }

echo "OK, smoke test passed"
