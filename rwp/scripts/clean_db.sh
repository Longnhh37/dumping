#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT"

if [[ $# -gt 1 || ( $# -eq 1 && "$1" != "--yes" ) ]]; then
  echo "Usage: $0 [--yes]" >&2
  exit 2
fi

if [[ -f "$ROOT/.env" ]]; then
  set -a
  # .env is maintained as shell-compatible KEY=value configuration.
  # shellcheck disable=SC1091
  source "$ROOT/.env"
  set +a
fi

if ! command -v psql >/dev/null 2>&1; then
  echo "clean_db requires psql (PostgreSQL client tools)" >&2
  exit 1
fi

: "${TO_DO_DB_URL:?Set TO_DO_DB_URL in .env or the environment}"
: "${AUTH_DB_URL:?Set AUTH_DB_URL in .env or the environment}"

todo_info="$(psql "$TO_DO_DB_URL" -X -v ON_ERROR_STOP=1 -Atqc \
  "SELECT current_database() || ' @ ' || COALESCE(inet_server_addr()::text, 'local') || ':' || COALESCE(inet_server_port()::text, '')")"
auth_info="$(psql "$AUTH_DB_URL" -X -v ON_ERROR_STOP=1 -Atqc \
  "SELECT current_database() || ' @ ' || COALESCE(inet_server_addr()::text, 'local') || ':' || COALESCE(inet_server_port()::text, '')")"

echo "This will permanently delete all application rows from these PostgreSQL databases:"
echo "  to-do: $todo_info"
echo "  auth:  $auth_info"
echo "Table schemas and _sqlx_migrations records will be preserved. Sequences will restart."

if [[ "${1:-}" != "--yes" ]]; then
  read -r -p "Type CLEAN to_do auth to continue: " confirmation
  if [[ "$confirmation" != "CLEAN to_do auth" ]]; then
    echo "Cancelled; no data was changed."
    exit 1
  fi
fi

TRUNCATE_SQL="$(cat <<'SQL'
DO $$
DECLARE
    tables_to_truncate TEXT;
BEGIN
    SELECT string_agg(format('%I.%I', schemaname, tablename), ', ')
      INTO tables_to_truncate
      FROM pg_tables
     WHERE schemaname = 'public'
       AND tablename <> '_sqlx_migrations';

    IF tables_to_truncate IS NOT NULL THEN
        EXECUTE 'TRUNCATE TABLE ' || tables_to_truncate || ' RESTART IDENTITY CASCADE';
    END IF;
END
$$;
SQL
)"

psql "$TO_DO_DB_URL" -X -v ON_ERROR_STOP=1 -c "$TRUNCATE_SQL"
psql "$AUTH_DB_URL" -X -v ON_ERROR_STOP=1 -c "$TRUNCATE_SQL"

echo "Database rows cleared; schemas and migration history were preserved."
