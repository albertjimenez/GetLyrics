#!/bin/bash

# Script: build-local.sh
# Description: Simple script to build Docker image for current architecture

set -e

TAG="${1:-getlyrics:latest}"

# Detect architecture
ARCH=$(uname -m)
case $ARCH in
    x86_64)
        TARGETARCH="amd64"
        ;;
    aarch64|arm64)
        TARGETARCH="arm64"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Handle macOS Apple Silicon with Rosetta
if [[ "$(uname -s)" == "Darwin" ]] && [[ "$ARCH" == "x86_64" ]]; then
    if command -v sysctl > /dev/null && [[ $(sysctl -n sysctl.proc_translated 2>/dev/null) -eq 1 ]]; then
        echo "Apple Silicon detected (running under Rosetta)"
        TARGETARCH="arm64"
    fi
fi

echo "Building for architecture: $TARGETARCH"
echo "Tag: $TAG"

# Build the image
docker build \
    --build-arg TARGETARCH="$TARGETARCH" \
    -t "$TAG" \
    .

echo ""
echo "Build completed!"
echo "Run with: docker run --rm $TAG"