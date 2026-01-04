
FROM rust:1.78-slim as builder
WORKDIR /app
COPY ../../core-engine /app/core-engine
WORKDIR /app/core-engine
RUN cargo build -p m0d --release

FROM debian:bookworm-slim
RUN useradd -m appuser
COPY --from=builder /app/core-engine/target/release/m0d /usr/local/bin/m0d
USER appuser
EXPOSE 9100
ENTRYPOINT ["m0d"]
