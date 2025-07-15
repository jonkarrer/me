FROM lukemathwalker/cargo-chef:latest-rust-slim-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime

RUN --mount=type=secret,id=APP_PORT \
    --mount=type=secret,id=TURSO_AUTH_TOKEN \
    --mount=type=secret,id=TURSO_DATABASE_URL \
    APP_PORT="$(cat /run/secrets/APP_PORT)" \
    TURSO_AUTH_TOKEN="$(cat /run/secrets/TURSO_AUTH_TOKEN)" \
    TURSO_DATABASE_URL="$(cat /run/secrets/TURSO_DATABASE_URL)"

RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates

COPY --from=builder /app/frontend /frontend
COPY --from=builder /app/target/release/me /

ENTRYPOINT ["/me"]