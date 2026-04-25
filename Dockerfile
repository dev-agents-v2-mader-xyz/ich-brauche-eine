## Stage 1: Build WASM frontend
FROM rust:1.88-slim AS wasm-builder
RUN apt-get update && apt-get install -y curl pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli@0.2.118
WORKDIR /app
COPY . .
WORKDIR /app/crates/frontend
RUN trunk build --release --public-url /

## Stage 2: Build backend
FROM rust:1.88-slim AS backend-builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY . .
COPY --from=wasm-builder /app/dist /app/dist
ENV SQLX_OFFLINE=true
RUN cargo build --release -p backend

## Stage 3: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 curl && rm -rf /var/lib/apt/lists/*
RUN useradd -m -u 1000 app
USER app
WORKDIR /app
COPY --from=backend-builder /app/target/release/backend /app/backend
COPY --from=wasm-builder /app/dist /app/dist
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
EXPOSE 8000

HEALTHCHECK --interval=30s --timeout=5s --start-period=15s --retries=3 \
    CMD curl -sf http://localhost:8000/health || exit 1

ENTRYPOINT ["/app/backend"]
