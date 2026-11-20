# Rust Web Programming

A rust web following Maxwell Flitton's _Rust Web Programming_.

## Architecture

```text
 Browser
   | HTTP: login and to-do API requests (JWT in the `token` header)
   v
+-------------------------- ingress process ----------------------------+
|  Frontend assets       /api/v1/auth/*          /api/v1/{get,create,   |
|  (embedded files)       |                      update,delete}         |
|                         v                           v                 |
|                 Auth Axum router              To-do Axum router       |
+-------------------------|---------------------------|-----------------+
                          |                           |
                  auth-core layer             GetUserSession
                          |                    (auth-kernel)
                  auth-dal layer                      |
                          |                    cache-client
                          v                           |
                  Auth PostgreSQL DB          Redis `session.*` commands
                                                      |
                                             Redis user-session module
                                                      |
                              on refresh: auth-kernel reloads the user
                              and logs the refreshed session into Redis
                                                      |
                                               to-do-core layer
                                                      |
                                               to-do-dal layer
                                                      v
                                               To-do PostgreSQL DB
```

Ingress currently assembles the frontend, auth router, and to-do router into
one Axum process. The routers call their core and data-access layers in-process;
the user-session cache is the Redis boundary. `to-do-axum-server` and
`auth-axum-server` also contain standalone server entry points.

### Components and layers

- **Frontend** (`frontend/`): React UI for login and to-do operations. It calls
  the HTTP API and uses `rust-interface/` WebAssembly for task action labels.
- **Ingress** (`ingress/`): the application entry point on port 8001. It embeds
  frontend assets, handles browser routes, and combines the auth and to-do API
  routers.
- **Networking / Axum** (`nanoservices/*/networking/axum_server/`): extracts
  request data and JWTs, maps routes to handlers, and formats HTTP responses.
- **Kernel / core** (`auth/kernel`, `auth/core`, `to_do/core`): coordinates
  authentication and session lookup, then applies service operations without
  handling HTTP routing or SQL details.
- **DAL** (`nanoservices/*/dal/`): owns database schemas, migrations, and
  PostgreSQL queries. Auth and to-do data use separate databases.
- **User-session cache** (`nanoservices/user-session-cache/`): `cache-client`
  sends session commands; `cache-module` implements them inside Redis. A
  to-do request resolves the JWT's unique ID to a cached numeric user ID. When
  the cache asks for a refresh, the auth kernel reloads the user and resets the
  cached session.
- **Glue** (`glue/`): shared JWT token, error, and database-pool utilities.

The normal request path is:

1. The browser sends a JWT with its API request to ingress.
2. The Axum handler validates/extracts the token and asks `GetUserSession` for
   the user's cached ID.
3. `cache-client` calls the Redis module. On refresh, the auth kernel looks up
   the user through the configured auth backend and writes a new cache entry.
4. The handler passes the user ID to `to-do-core`; the core calls `to-do-dal`,
   which reads or changes the user's rows in PostgreSQL.

Auth can resolve users through the core PostgreSQL backend or the HTTP backend,
selected by the `auth-core-postgres` or `auth-http` feature.

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
