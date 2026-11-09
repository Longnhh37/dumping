#!/usr/bin/env zsh
# File: scripts/basic_test.sh
# Smoke test for the axum to-do server.
# Requires: cargo, xh, jq.
# WARNING: this resets ./tasks.json at the workspace root to "{}".

set -euo pipefail

# Run everything from the workspace root (parent of scripts/)
cd "${0:A:h}/.."

OUTPUT=basic_test_output.txt
BIN="${CARGO_TARGET_DIR:-target}/debug/to-do-axum-server"
BASE=":8080/api/v1"

# Build synchronously first, so $! below is the PID of the server itself
echo "Building axum server..."
cargo build -p to-do-axum-server

# Start from a clean state
echo '{}' > tasks.json
: > "$OUTPUT"

# Always stop the server, even when a step fails
PID=""
cleanup() {
  [[ -n "$PID" ]] && kill "$PID" 2>/dev/null || true
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

step "create writing" POST "$BASE/create" title=writing status=PENDING
step "create coding"  POST "$BASE/create" title=coding  status=PENDING
step "get all"        GET  "$BASE/get/all"
step "get by name"    GET  "$BASE/get/writing"
step "delete coding"  DELETE "$BASE/delete/coding"

# update requires the `token` header (HeaderToken extractor)
step "update writing" PATCH "$BASE/update" token:some-token title=writing status=DONE

# update without a token must be rejected with 401
echo "update without token (expect 401)"
if xh --ignore-stdin --check-status PATCH "$BASE/update" title=writing status=DONE &>/dev/null; then
  echo "FAIL: update without token was accepted" >&2
  exit 1
fi

# Final state: "writing" is done, nothing is pending
xh --ignore-stdin GET "$BASE/get/all" \
  | jq -e '(.done | length == 1) and (.pending | length == 0)' >/dev/null \
  || { echo "FAIL: unexpected final state" >&2; exit 1; }

echo "OK, details in $OUTPUT"
