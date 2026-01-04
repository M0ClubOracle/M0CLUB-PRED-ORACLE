
FROM rust:1.78-slim as builder
WORKDIR /app
COPY ../../core-engine /app/core-engine
WORKDIR /app/core-engine
RUN cargo build -p m0-signer-agent --release

FROM debian:bookworm-slim
RUN useradd -m appuser
COPY --from=builder /app/core-engine/target/release/m0-signer-agent /usr/local/bin/m0-signer-agent
USER appuser
ENTRYPOINT ["m0-signer-agent"]
