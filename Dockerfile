FROM rust:alpine AS builder

WORKDIR /app

COPY . .
RUN cargo build --release

FROM alpine:latest
# Install Doppler CLI
RUN wget -q -t3 'https://packages.doppler.com/public/cli/rsa.8004D9FF50437357.key' -O /etc/apk/keys/cli@doppler-8004D9FF50437357.rsa.pub && \
    echo 'https://packages.doppler.com/public/cli/alpine/any-version/main' | tee -a /etc/apk/repositories && \
    apk add doppler
COPY --from=builder /app/targer/release/me /
CMD ["doppler", "run", "--", "/me"]