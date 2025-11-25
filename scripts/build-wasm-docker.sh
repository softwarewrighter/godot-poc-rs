#!/usr/bin/env bash
#
# build-wasm-docker.sh - Build WASM library using Docker
#
# This builds the Rust library for WASM using a Linux Docker container
# to avoid macOS cross-compilation issues with godot-ffi bindgen.
#
# Prerequisites:
#   - Docker must be installed and running
#
# Usage:
#   ./scripts/build-wasm-docker.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Check prerequisites
check_docker() {
    if ! command -v docker &> /dev/null; then
        error "Docker not found. Please install Docker to build WASM."
    fi

    if ! docker info &> /dev/null; then
        error "Docker daemon is not running. Please start Docker."
    fi

    info "Docker is available"
}

# Build the Docker image
build_image() {
    info "Building Docker image for WASM compilation..."

    cd "$PROJECT_ROOT"

    docker build -f Dockerfile.wasm -t godot-poc-rs-wasm .
}

# Extract the WASM file
extract_wasm() {
    info "Extracting WASM library..."

    local container_id
    container_id=$(docker create godot-poc-rs-wasm)

    mkdir -p "$PROJECT_ROOT/target/wasm32-unknown-emscripten/release"

    docker cp "$container_id:/app/target/wasm32-unknown-emscripten/release/libgodot_poc_rs.wasm" \
        "$PROJECT_ROOT/target/wasm32-unknown-emscripten/release/" 2>/dev/null || \
    docker cp "$container_id:/app/target/wasm32-unknown-emscripten/release/godot_poc_rs.wasm" \
        "$PROJECT_ROOT/target/wasm32-unknown-emscripten/release/"

    docker rm "$container_id" > /dev/null

    info "WASM library extracted to: target/wasm32-unknown-emscripten/release/"
}

# Main
main() {
    info "Building WASM using Docker (to avoid macOS cross-compilation issues)"
    info ""

    check_docker
    build_image
    extract_wasm

    info ""
    info "WASM build complete!"
    info "Now run: ./scripts/export-to-web.sh --skip-rust"
}

main "$@"
