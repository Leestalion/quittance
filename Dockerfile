# Stage 1: Build frontend
FROM node:20-alpine AS frontend
WORKDIR /frontend
COPY frontend/package*.json ./
RUN npm ci
COPY frontend ./
RUN npm run build

# Stage 2: Build backend
FROM lukemathwalker/cargo-chef:latest-rust-1-bookworm AS chef

# Create and change to the app directory.
WORKDIR /app

# Stage 3: Copy source code to the container
FROM chef AS planner
COPY backend ./backend
WORKDIR /app/backend
RUN cargo chef prepare --recipe-path recipe.json

# Stage 4: Build the application
FROM chef AS builder
WORKDIR /app/backend
COPY --from=planner /app/backend/recipe.json recipe.json

# Set SQLX_OFFLINE to true for offline SQLx compilation
ENV SQLX_OFFLINE=true

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY backend/ .
RUN cargo build --release

# Stage 5: Runtime - minimal image
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Install runtime dependencies only
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    wkhtmltopdf \
    && rm -rf /var/lib/apt/lists/*

# Copy only the compiled binary
COPY --from=builder /app/backend/target/release/quittance /usr/local/bin/quittance

# Copy legal templates (read at runtime by the server-side PDF renderer)
COPY backend/src/legal_templates /app/legal_templates

# Copy frontend dist
COPY --from=frontend /frontend/dist /app/frontend/dist

ENV FRONTEND_PATH=/app/frontend/dist
ENV LEGAL_TEMPLATES_DIR=/app/legal_templates
ENV WKHTMLTOPDF_PATH=/usr/bin/wkhtmltopdf
ENV RUST_LOG=info

EXPOSE 8080

CMD ["quittance"]
