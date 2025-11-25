#!/usr/bin/env bash
#
# build.sh - Build the Rust library for Godot
#
# Usage:
#   ./scripts/build.sh          # Debug build (default)
#   ./scripts/build.sh release  # Release build
#   ./scripts/build.sh clean    # Clean build artifacts

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

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
check_prerequisites() {
    info "Checking prerequisites..."

    if ! command -v cargo &> /dev/null; then
        error "Rust/Cargo not found. Install from https://rustup.rs"
    fi

    if ! command -v rustc &> /dev/null; then
        error "Rust compiler not found. Install from https://rustup.rs"
    fi

    info "Rust version: $(rustc --version)"
    info "Cargo version: $(cargo --version)"
}

# Build native library (for development/testing)
build_native() {
    local build_type="${1:-debug}"

    info "Building native Rust library ($build_type)..."

    cd "$PROJECT_ROOT"

    if [[ "$build_type" == "release" ]]; then
        cargo build --release
        info "Built release library at: target/release/"
    else
        cargo build
        info "Built debug library at: target/debug/"
    fi
}

# Clean build artifacts
clean() {
    info "Cleaning build artifacts..."
    cd "$PROJECT_ROOT"
    cargo clean
    info "Clean complete"
}

# Main
main() {
    local command="${1:-debug}"

    case "$command" in
        debug)
            check_prerequisites
            build_native "debug"
            ;;
        release)
            check_prerequisites
            build_native "release"
            ;;
        clean)
            clean
            ;;
        *)
            echo "Usage: $0 [debug|release|clean]"
            echo ""
            echo "Commands:"
            echo "  debug   - Build debug version (default)"
            echo "  release - Build release version"
            echo "  clean   - Clean build artifacts"
            exit 1
            ;;
    esac

    info "Build complete!"
}

main "$@"
