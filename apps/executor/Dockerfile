FROM rust:1.90-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p executor

FROM debian:bookworm-slim
RUN useradd --create-home --uid 1000 appuser
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates pkg-config libssl3 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/executor /usr/local/bin/executor
COPY content /app/content
USER appuser
EXPOSE 8082
ENTRYPOINT ["/usr/local/bin/executor"]
