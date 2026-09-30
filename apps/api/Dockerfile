FROM rust:1.90-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p api

FROM debian:bookworm-slim
RUN useradd --create-home --uid 1000 appuser \
    && apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/api /usr/local/bin/api
COPY content /app/content
USER appuser
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/api"]
