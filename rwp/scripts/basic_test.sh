#!/usr/bin/env zsh
# File: scripts/basic_test.sh
# Smoke test for the PostgreSQL-backed axum to-do server.
# Requires: cargo, xh, jq.

set -euo pipefail

# Run everything from the workspace root (parent of scripts/)
cd "${0:A:h}/.."

OUTPUT=basic_test_output.txt
BIN="${CARGO_TARGET_DIR:-target}/debug/to-do-axum-server"
BASE=":8080/api/v1"
RUN_ID="${$}-$(date +%s)"
WRITING="smoke-writing-${RUN_ID}"
CODING="smoke-coding-${RUN_ID}"

# Build synchronously first, so $! below is the PID of the server itself
echo "Building axum server..."
cargo build -p to-do-axum-server

: > "$OUTPUT"

# Always stop the server, even when a step fails
PID=""
cleanup() {
  if [[ -n "$PID" ]]; then
    xh --ignore-stdin DELETE "$BASE/delete/$WRITING" &>/dev/null || true
    xh --ignore-stdin DELETE "$BASE/delete/$CODING" &>/dev/null || true
    kill "$PID" 2>/dev/null || true
    wait "$PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT

"$BIN" &
PID=$!
echo "Server started with PID: $PID"

# Wait until the server answers instead of a blind sleep
for _ in {1..30}; do
  xh --ignore-stdin --check-status GET "$BASE/get/all" &>/dev/null && break
  sleep 0.2
done

# Run one request, log its JSON body to $OUTPUT, fail on non-2xx
step() {
  local label=$1; shift
  echo "$label"
  echo "### $label" >> "$OUTPUT"
  xh --ignore-stdin --check-status "$@" | jq . >> "$OUTPUT"
}

step "create writing" POST "$BASE/create" name="$WRITING" status=PENDING
step "create coding"  POST "$BASE/create" name="$CODING" status=PENDING
step "get all"        GET  "$BASE/get/all"
step "get by name"    GET  "$BASE/get/$WRITING"
step "delete coding"  DELETE "$BASE/delete/$CODING"

# update requires the `token` header (HeaderToken extractor)
step "update writing" PATCH "$BASE/update" token:some-token name="$WRITING" status=DONE

# update without a token must be rejected with 401
echo "update without token (expect 401)"
if xh --ignore-stdin --check-status PATCH "$BASE/update" name="$WRITING" status=DONE &>/dev/null; then
  echo "FAIL: update without token was accepted" >&2
  exit 1
fi

# Final state: "writing" is done, nothing is pending
xh --ignore-stdin GET "$BASE/get/all" \
  | jq -e --arg writing "$WRITING" \
    'any(.done[]; .name == $writing and .status == "DONE") and all(.pending[]; .name != $writing)' >/dev/null \
  || { echo "FAIL: unexpected final state" >&2; exit 1; }

echo "OK, details in $OUTPUT"
