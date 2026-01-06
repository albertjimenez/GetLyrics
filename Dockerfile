# Stage 1: Build the application with musl
FROM rust:slim-bookworm AS builder
ARG TARGETARCH
ARG X86_NAME=x86_64-unknown-linux-musl
ARG ARM64_NAME=aarch64-unknown-linux-musl
# Install musl target and necessary dependencies
RUN rustup target add $X86_NAME $ARM64_NAME  && \
    apt-get update && apt-get install -y musl-tools llvm clang pkg-config libssl-dev perl make

WORKDIR /app

# Copy Cargo files and download dependencies
COPY Cargo.toml Cargo.lock ./
# Copy source code and build
COPY src ./src
RUN if [ "$TARGETARCH" = "amd64" ]; then \
    cargo build --release --target $X86_NAME && \
    mv target/$X86_NAME/release/getlyrics .; \
    elif [ "$TARGETARCH" = "arm64" ]; then \
    cargo build --release --target $ARM64_NAME && \
    mv target/$ARM64_NAME/release/getlyrics .; \
    fi
RUN strip getlyrics



# Stage 2: Create a minimal musl-based image
FROM scratch
# Copy the musl binary from the builder stage
COPY --from=builder /app/getlyrics /getlyrics
# Set the entry point
ENTRYPOINT ["/getlyrics"]