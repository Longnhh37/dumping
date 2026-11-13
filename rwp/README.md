# Rust Web Programming

A rust web following Maxwell Flitton's *Rust Web Programming*.

## Run locally

Run `./scripts/init.sh` from the repository root. It creates `.env` from
`.env.example` when needed, starts PostgreSQL with Docker Compose, waits for its
health check, builds the frontend, and starts ingress on port 8001.

## Test

Run `./scripts/test.sh` for Rust unit and in-memory API/ingress integration
tests, frontend URL tests, TypeScript checking, and frontend build.

To include the PostgreSQL-backed API smoke test, make sure PostgreSQL is running
and run `RUN_POSTGRES_INTEGRATION=1 ./scripts/test.sh`. This starts the backend
server on the configured `TO_DO_WEB_PORT` (8080 by default).
