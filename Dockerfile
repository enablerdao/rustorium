# Build stage
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
WORKDIR /usr/src/rustorium
COPY . .

# Build the project
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create rustorium user
RUN useradd -r -s /bin/false rustorium

# Create necessary directories
RUN mkdir -p /etc/rustorium /var/lib/rustorium /var/log/rustorium \
    && chown -R rustorium:rustorium /etc/rustorium /var/lib/rustorium /var/log/rustorium \
    && chmod 700 /var/lib/rustorium

# Copy the binary from builder
COPY --from=builder /usr/src/rustorium/target/release/rustorium /usr/local/bin/
RUN chmod +x /usr/local/bin/rustorium

# Copy default configuration
COPY config/production.toml.example /etc/rustorium/config.toml
RUN chown rustorium:rustorium /etc/rustorium/config.toml

# Set working directory
WORKDIR /var/lib/rustorium

# Expose ports
EXPOSE 9070 9071 9072

# Switch to rustorium user
USER rustorium

# Set environment variables
ENV RUST_LOG=info
ENV RUSTORIUM_CONFIG=/etc/rustorium/config.toml

# Health check
HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:9071/health || exit 1

# Start the node
ENTRYPOINT ["rustorium"]
CMD ["--config", "/etc/rustorium/config.toml"]
