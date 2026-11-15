# Rust Web Programming

A rust web following Maxwell Flitton's *Rust Web Programming*.

## Run locally

Run `./scripts/init.sh` from the repository root. It creates `.env` from
`.env.example` when needed, starts PostgreSQL with Docker Compose, builds the
frontend, and starts ingress on port 8001. The container initializes separate
`to_do` and `auth` databases; ingress runs each service's migrations at startup.
Configure their URLs and pool sizes with `TO_DO_DB_URL`,
`TO_DO_MAX_CONNECTIONS`, `AUTH_DB_URL`, and `AUTH_MAX_CONNECTIONS` in `.env`.
If `.env` was created before these settings were added, set both max-connection
values to `5`.
Run `./scripts/cleanup.sh` to stop the local PostgreSQL container while
preserving its database data.

## Test

Run `./scripts/test.sh` for Rust unit and in-memory API/ingress integration
tests, frontend URL tests, TypeScript checking, and frontend build.

To include the PostgreSQL-backed API smoke test, make sure PostgreSQL is running
and run `RUN_POSTGRES_INTEGRATION=1 ./scripts/test.sh`. This starts the backend
server on the configured `TO_DO_WEB_PORT` (8080 by default).
