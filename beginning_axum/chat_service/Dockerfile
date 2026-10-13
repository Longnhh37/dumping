FROM oven/bun:1 AS frontend
WORKDIR /app/frontend
COPY frontend/package.json frontend/bun.lock ./
RUN bun install --frozen-lockfile
COPY frontend/ ./
RUN bun run build

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY backend/ ./backend/
RUN cargo chef prepare --manifest-path backend/Cargo.toml --recipe-path recipe.json

FROM chef AS backend
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --manifest-path backend/Cargo.toml --recipe-path recipe.json
COPY backend/ ./backend/
RUN cargo build --release --manifest-path backend/Cargo.toml --bin backend

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=backend /app/backend/target/release/backend ./app
COPY --from=frontend /app/frontend/dist ./backend/static
EXPOSE 3000
ENTRYPOINT ["./app"]
