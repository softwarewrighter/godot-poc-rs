#!/usr/bin/env bash
#
# export-to-macos.sh - Export Godot game as macOS application
#
# This script:
# 1. Builds the Rust library for macOS (release)
# 2. Exports the Godot project as a macOS app bundle or DMG
#
# Usage:
#   ./scripts/export-to-macos.sh          # Build and export as DMG
#   ./scripts/export-to-macos.sh --app    # Export as .app bundle only
#   ./scripts/export-to-macos.sh --skip-rust  # Skip Rust build

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
GODOT_PROJECT="$PROJECT_ROOT/godot"
EXPORT_DIR="$PROJECT_ROOT/export/macos"

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

    info "Rust version: $(rustc --version)"
    info "Godot version: $(godot --version 2>&1 || echo 'unknown')"
}

# Build Rust library
build_rust() {
    info "Building Rust library for macOS (release)..."

    cd "$PROJECT_ROOT"
    cargo build --release

    info "Library built at: target/release/libgodot_poc_rs.dylib"
}

# Export Godot project for macOS
export_godot() {
    local export_format="$1"

    info "Exporting Godot project for macOS..."

    # Create export directory
    mkdir -p "$EXPORT_DIR"

    # Check if Godot project exists
    if [[ ! -f "$GODOT_PROJECT/project.godot" ]]; then
        error "Godot project not found at $GODOT_PROJECT/project.godot"
    fi

    cd "$GODOT_PROJECT"

    # First import the project (needed for first-time exports)
    info "Importing Godot project..."
    godot --headless --import . 2>/dev/null || true

    # Export to macOS
    if [[ "$export_format" == "app" ]]; then
        info "Exporting as .app bundle..."
        # For .app export, we need to use a .zip extension which Godot extracts
        godot --headless --export-release "macOS" "$EXPORT_DIR/revolving-match3.zip"

        # Extract the zip to get the .app bundle
        if [[ -f "$EXPORT_DIR/revolving-match3.zip" ]]; then
            cd "$EXPORT_DIR"
            unzip -o revolving-match3.zip
            rm revolving-match3.zip
            info "App bundle exported at: $EXPORT_DIR/revolving-match3.app"
        fi
    else
        info "Exporting as DMG..."
        godot --headless --export-release "macOS" "$EXPORT_DIR/revolving-match3.dmg"
        info "DMG exported at: $EXPORT_DIR/revolving-match3.dmg"
    fi

    info "Export complete at: $EXPORT_DIR/"
}

# Main
main() {
    local skip_rust=false
    local export_format="dmg"

    # Parse arguments
    for arg in "$@"; do
        case "$arg" in
            --skip-rust)
                skip_rust=true
                ;;
            --app)
                export_format="app"
                ;;
            --help|-h)
                echo "Usage: $0 [--skip-rust] [--app]"
                echo ""
                echo "Options:"
                echo "  --skip-rust  Skip Rust build (use existing library)"
                echo "  --app        Export as .app bundle instead of DMG"
                echo ""
                echo "This script builds the Rust library and exports"
                echo "the Godot project as a macOS application."
                exit 0
                ;;
            *)
                warn "Unknown argument: $arg"
                ;;
        esac
    done

    check_prerequisites

    if [[ "$skip_rust" == false ]]; then
        build_rust
    else
        info "Skipping Rust build (--skip-rust specified)"
    fi

    export_godot "$export_format"

    info ""
    info "macOS export complete!"
    info ""
    info "Files are in: $EXPORT_DIR/"
    if [[ "$export_format" == "app" ]]; then
        info "Run: open $EXPORT_DIR/revolving-match3.app"
    else
        info "Mount: open $EXPORT_DIR/revolving-match3.dmg"
    fi
}

main "$@"
