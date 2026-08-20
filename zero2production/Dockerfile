
# =============================================================
# Helper
# =============================================================
FROM rust:1.94 AS chef

RUN cargo install cargo-chef

# =============================================================
# Planner
# =============================================================
FROM chef AS planner

WORKDIR /app

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

# =============================================================
# Builder
# =============================================================
FROM chef AS builder

WORKDIR /app

# native deps
RUN apt-get update && \
	apt-get install -y --no-install-recommends \
		lld \
		clang && \
	rm -rf /var/lib/apt/lists/*

ENV SQLX_OFFLINE=true

COPY --from=planner /app/recipe.json recipe.json

# compile dependencies only
RUN cargo chef cook \
	--release \
	--recipe-path recipe.json \
	--locked

# copy actual source code
COPY . .

RUN cargo build --release --locked

# =============================================================
# Runtime
# =============================================================
FROM debian:bullseye-slim AS runtime

WORKDIR /app

RUN apt-get update && \
	apt-get install -y --no-install-recommends \
		openssl \
		ca-certificates && \
	rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zero2prod .
COPY configuration configuration

ENV APP_ENVIRONMENT=production

ENTRYPOINT ["./zero2prod"]
