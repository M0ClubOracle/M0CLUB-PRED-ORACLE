
FROM rust:1.78-slim as builder
WORKDIR /app
COPY ../../services /app/services
WORKDIR /app/services
RUN cargo build -p m0-indexer --release

FROM debian:bookworm-slim
RUN useradd -m appuser
COPY --from=builder /app/services/target/release/m0-indexer /usr/local/bin/m0-indexer
USER appuser
ENTRYPOINT ["m0-indexer"]
