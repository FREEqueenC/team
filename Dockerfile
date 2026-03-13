# Multi-stage Dockerfile for production-ready Rust API
# Stage 1: Build the application
FROM rust:1.91-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Build the application in release mode
RUN cargo build --release --bin partner_tools

# Stage 2: Create a slim runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/partner_tools /app/partner_tools
# Copy any necessary runtime configuration or static files
COPY config/ /app/config/
COPY admin/ /app/admin/
COPY projects/ /app/projects/

# Expose the API port (Cloud Run will inject $PORT, but the app uses 8081)
EXPOSE 8081

# Run the API server
# We use the 'serve' command as specified in the original Dockerfile
CMD ["/app/partner_tools", "serve"]