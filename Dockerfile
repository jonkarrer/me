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
RUN sudo apt install -y gcc-aarch64-linux-gnu
RUN rustup target add aarch64-unknown-linux-gnu
RUN cargo build --release --target=aarch64-unknown-linux-gnu

# We do not need the Rust toolchain to run the binary!
FROM arm64v8/debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y curl && apt-get clean
RUN curl -Ls --tlsv1.2 --proto "=https" --retry 3 https://cli.doppler.com/install.sh | sh

COPY --from=builder /app/frontend /frontend
COPY --from=builder /app/target/aarch64-unknown-linux-gnu/release/me /me


EXPOSE 5105
CMD ["doppler", "run", "--", "/me"]

# FROM arm64v8/debian:bullseye-slim
# COPY ./target/aarch64-unknown-linux-gnu/release/me /
# COPY frontend /frontend
# ENTRYPOINT [ "/me" ]