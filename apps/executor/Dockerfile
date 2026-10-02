FROM rust:1.90-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p executor

FROM rust:1.90-bookworm
RUN useradd --create-home --uid 1000 appuser
WORKDIR /app
COPY --from=builder /app/target/release/executor /usr/local/bin/executor
COPY content /app/content
USER appuser
EXPOSE 8082
ENTRYPOINT ["/usr/local/bin/executor"]
