
FROM rust:1.78-slim as builder
WORKDIR /app
COPY ../../services /app/services
WORKDIR /app/services
RUN cargo build -p m0-api-gateway --release

FROM debian:bookworm-slim
RUN useradd -m appuser
COPY --from=builder /app/services/target/release/m0-api-gateway /usr/local/bin/m0-api-gateway
USER appuser
EXPOSE 8080
ENTRYPOINT ["m0-api-gateway"]
