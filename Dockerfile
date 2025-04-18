# Stage 1: Build the application with musl
FROM rust:slim-bookworm AS builder

# Install musl target and necessary dependencies
RUN rustup target add x86_64-unknown-linux-musl && \
    apt-get update && apt-get install -y musl-tools llvm clang pkg-config libssl-dev perl make

WORKDIR /app

# Copy Cargo files and download dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --target x86_64-unknown-linux-musl

# Copy source code and build
COPY src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 2: Create a minimal musl-based image
FROM scratch
# Copy the musl binary from the builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/getlyrics /getlyrics
# Set the entry point
ENTRYPOINT ["/getlyrics"]