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

### Prerequisites

- Docker with the Docker Compose plugin
- Rust toolchain and Cargo
- [Bun](https://bun.sh/) for building and serving the frontend
- `xh` to use the user creation helper

### Start the app

1. From the repository root, initialize configuration and start the services:

   ```sh
   ./scripts/init.sh
   ```

   The script copies `.env.example` to `.env` if needed, starts PostgreSQL,
   Redis, and Elasticsearch through Docker Compose, builds the frontend, and
   starts ingress on port `8001`. Ingress runs the auth and to-do database
   migrations on startup. Set a private `JWT_SECRET` in `.env` before using the
   app. The default local cache URL is `redis://127.0.0.1:6379`.

2. In another terminal, register a user:

   ```sh
   ./scripts/create_user user@example.com 'a-strong-password'
   ```

   The helper posts to `http://127.0.0.1:8001/api/v1/users/create`. Override
   the base URL with `INGRESS_URL` if ingress is listening elsewhere. The
   account is stored in the auth PostgreSQL database.

3. Open <http://127.0.0.1:8001> and sign in with that email and password. The
   app stores the returned JWT in browser storage, uses it for to-do requests,
   and clears the session when you log out. To-do items are scoped to the
   signed-in user.

4. Stop the local stack when finished:

   ```sh
   ./scripts/cleanup.sh
   ```

   This stops the containers and preserves their database data.

## Test

Run `./scripts/test.sh` for Rust unit tests (including Basic-auth parsing), the
in-memory to-do API lifecycle integration test, ingress routing tests, frontend
URL tests, TypeScript checking, and frontend build.

To include the PostgreSQL-backed API smoke test, start the local stack and sign
in to get a JWT. For example, with `xh` and `jq` installed:

```sh
export TOKEN="$(xh --ignore-stdin -a user@example.com:a-strong-password POST :8001/api/v1/auth/login | jq -r .)"
RUN_POSTGRES_INTEGRATION=1 ./scripts/test.sh
```

The smoke test starts the standalone to-do server on `TO_DO_WEB_PORT` (8080 by
default). Its current API uses `title`, numeric item IDs, and `/get/{id}` and
`/delete/{id}` routes.

### API flow

The browser calls `POST /api/v1/auth/login` with Basic auth. Ingress validates
the credentials, returns a JWT, and writes the session to Redis. To-do requests
send that JWT in the `token` header; the API resolves its unique ID through the
session cache and scopes CRUD operations by the resulting user ID. Logout calls
`POST /api/v1/auth/logout`, which removes the cached session.
