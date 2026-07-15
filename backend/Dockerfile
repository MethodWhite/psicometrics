# Stage 1 — Build
FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>/dev/null || true
RUN rm -rf src

COPY . .
RUN cargo build --release

# Stage 2 — Runtime
FROM debian:bookworm-slim AS runtime

RUN groupadd -r psicometrics && useradd -r -g psicometrics -u 1000 psicometrics
RUN apt-get update && \
    apt-get install -y ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/psicometrics-backend /app/psicometrics-backend
COPY --from=builder /app/data/ /app/data/

RUN chown -R psicometrics:psicometrics /app

USER psicometrics

EXPOSE 8000

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8000/api/v1/health || exit 1

CMD ["/app/psicometrics-backend"]
