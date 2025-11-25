#!/usr/bin/env bash
#
# export-to-web.sh - Export Godot game to WebAssembly
#
# This script:
# 1. Builds the Rust library for wasm32-unknown-emscripten
# 2. Exports the Godot project for web
# 3. Configures PWA and headers
#
# Usage:
#   ./scripts/export-to-web.sh          # Build and export
#   ./scripts/export-to-web.sh --skip-rust  # Skip Rust build (use existing)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
GODOT_PROJECT="$PROJECT_ROOT/godot"
EXPORT_DIR="$PROJECT_ROOT/export/web"

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

    if ! command -v godot &> /dev/null; then
        error "Godot not found. Install Godot 4.x and ensure it's in PATH"
    fi

    # Check for Emscripten (required for gdext web builds)
    if ! command -v emcc &> /dev/null; then
        error "Emscripten not found. Install it with:

    # Install Emscripten SDK
    git clone https://github.com/emscripten-core/emsdk.git
    cd emsdk
    ./emsdk install latest
    ./emsdk activate latest
    source ./emsdk_env.sh

    # Add to your shell profile for persistence"
    fi

    # Check for wasm target (wasm32-unknown-emscripten for gdext)
    if ! rustup target list --installed | grep -q "wasm32-unknown-emscripten"; then
        warn "wasm32-unknown-emscripten target not installed. Installing..."
        rustup target add wasm32-unknown-emscripten
    fi

    info "Rust version: $(rustc --version)"
    info "Godot version: $(godot --version 2>&1 || echo 'unknown')"
    info "Emscripten version: $(emcc --version | head -1)"
}

# Build Rust library for WebAssembly
build_wasm() {
    info "Building Rust library for WebAssembly (nothreads)..."

    cd "$PROJECT_ROOT"

    # Ensure nightly toolchain with rust-src is installed
    if ! rustup run nightly rustc --version &> /dev/null; then
        warn "Installing nightly toolchain..."
        rustup toolchain install nightly
    fi

    if ! rustup component list --toolchain nightly | grep -q "rust-src (installed)"; then
        warn "Installing rust-src for nightly..."
        rustup component add rust-src --toolchain nightly
    fi

    # Ensure wasm32-unknown-emscripten target is installed for nightly
    if ! rustup +nightly target list --installed | grep -q "wasm32-unknown-emscripten"; then
        warn "Installing wasm32-unknown-emscripten target for nightly..."
        rustup +nightly target add wasm32-unknown-emscripten
    fi

    # Build for wasm32-unknown-emscripten using nightly with build-std
    # Using nothreads variant to avoid pthread size mismatch issues
    # The -Zbuild-std flag rebuilds std for the target, required for WASM
    cargo +nightly build -Zbuild-std --release --target wasm32-unknown-emscripten

    info "WASM library built at: target/wasm32-unknown-emscripten/release/"
}

# Export Godot project for web
export_godot() {
    info "Exporting Godot project for web..."

    # Create export directory
    mkdir -p "$EXPORT_DIR"

    # Check if Godot project exists
    if [[ ! -f "$GODOT_PROJECT/project.godot" ]]; then
        error "Godot project not found at $GODOT_PROJECT/project.godot"
    fi

    # Export using Godot headless
    cd "$GODOT_PROJECT"

    # First import the project (needed for first-time exports)
    info "Importing Godot project..."
    godot --headless --import . 2>/dev/null || true

    # Export to web
    info "Exporting to web..."
    godot --headless --export-release "Web" "$EXPORT_DIR/index.html"

    info "Export complete at: $EXPORT_DIR/"
}

# Set up web configuration files
setup_web_config() {
    info "Setting up web configuration..."

    # Create _headers file for COOP/COEP (required for SharedArrayBuffer)
    cat > "$EXPORT_DIR/_headers" << 'EOF'
/*
  Cross-Origin-Opener-Policy: same-origin
  Cross-Origin-Embedder-Policy: require-corp
  Cross-Origin-Resource-Policy: cross-origin
EOF

    # Create vercel.json for Vercel deployment
    cat > "$EXPORT_DIR/vercel.json" << 'EOF'
{
  "headers": [
    {
      "source": "/(.*)",
      "headers": [
        { "key": "Cross-Origin-Opener-Policy", "value": "same-origin" },
        { "key": "Cross-Origin-Embedder-Policy", "value": "require-corp" },
        { "key": "Cross-Origin-Resource-Policy", "value": "cross-origin" }
      ]
    }
  ]
}
EOF

    info "Web configuration files created"
}

# Main
main() {
    local skip_rust=false

    # Parse arguments
    for arg in "$@"; do
        case "$arg" in
            --skip-rust)
                skip_rust=true
                ;;
            --help|-h)
                echo "Usage: $0 [--skip-rust]"
                echo ""
                echo "Options:"
                echo "  --skip-rust  Skip Rust build (use existing WASM library)"
                echo ""
                echo "This script builds the Rust library for WASM and exports"
                echo "the Godot project for web deployment."
                exit 0
                ;;
            *)
                warn "Unknown argument: $arg"
                ;;
        esac
    done

    check_prerequisites

    if [[ "$skip_rust" == false ]]; then
        build_wasm
    else
        info "Skipping Rust build (--skip-rust specified)"
    fi

    export_godot
    setup_web_config

    info "Web export complete!"
    info ""
    info "Files are in: $EXPORT_DIR/"
    info ""
    info "To test locally, run: ./scripts/serve-web.sh"
    info "To deploy to Vercel: cd $EXPORT_DIR && vercel"
}

main "$@"
