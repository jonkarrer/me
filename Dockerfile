FROM arm64v8/ubuntu:devel AS builder

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y curl build-essential && \
    apt-get clean

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

COPY . .

# # Install Doppler CLI
# RUN apt-get update && apt-get install -y apt-transport-https ca-certificates curl gnupg && \
#     curl -sLf --retry 3 --tlsv1.2 --proto "=https" 'https://packages.doppler.com/public/cli/gpg.DE2A7741A397C129.key' | gpg --dearmor -o /usr/share/keyrings/doppler-archive-keyring.gpg && \
#     echo "deb [signed-by=/usr/share/keyrings/doppler-archive-keyring.gpg] https://packages.doppler.com/public/cli/deb/debian any-version main" | tee /etc/apt/sources.list.d/doppler-cli.list && \
#     apt-get update && \
#     apt-get -y install doppler

RUN cargo build --release

FROM scratch
COPY --from=builder /app/target/release/me /
# CMD ["doppler", "run", "--", "/me"]
CMD ["/me"]