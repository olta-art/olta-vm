FROM rust:1-bookworm AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev

COPY Cargo.toml Cargo.lock ./
COPY . .

RUN cargo build --release --bin server

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates openssl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/server /usr/local/bin/server

ENV RUST_LOG=info
EXPOSE 8080
CMD ["server"]
