# Stage 1: Build the application with musl
FROM rust:1.97 AS builder
# Install musl targets and necessary dependencies.
# Multi-arch is delegated to GH Actions (buildx builds each platform natively),
# so `uname -m` always reports the platform currently being built — no arch branches needed.
RUN rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl && \
    apt-get update && apt-get install -y musl-tools llvm clang pkg-config libssl-dev perl make ca-certificates

WORKDIR /app

# Copy Cargo files and download dependencies
COPY Cargo.toml Cargo.lock ./
# Copy source code and build
COPY src ./src
RUN TARGET="$(uname -m | sed -e 's/^x86_64$/x86_64-unknown-linux-musl/' -e 's/^aarch64$/aarch64-unknown-linux-musl/')" && \
    cargo build --release --target "$TARGET" && \
    mv "target/$TARGET/release/getlyrics" /app/getlyrics
# Stage 2: Create a minimal musl-based image
FROM scratch
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
# Copy the musl binary from the builder stage
COPY --from=builder /app/target/release/getlyrics /getlyrics
# Set the entry point
ENTRYPOINT ["/getlyrics"]