#!/usr/bin/env bash
#
# build-wasm.sh - Build Rust library for WebAssembly
#
# This script sets up the proper environment for cross-compiling to WASM

set -euo pipefail

# Source Emscripten environment
if [[ -f "$HOME/emsdk/emsdk_env.sh" ]]; then
    source "$HOME/emsdk/emsdk_env.sh" > /dev/null 2>&1
elif [[ -f "$HOME/.emsdk/emsdk_env.sh" ]]; then
    source "$HOME/.emsdk/emsdk_env.sh" > /dev/null 2>&1
else
    echo "ERROR: Emscripten not found. Install from https://emscripten.org"
    exit 1
fi

echo "Using Emscripten: $(emcc --version | head -1)"

# Set up sysroot for bindgen
EMSDK_SYSROOT="${EMSDK}/upstream/emscripten/cache/sysroot"

if [[ ! -d "$EMSDK_SYSROOT" ]]; then
    echo "ERROR: Emscripten sysroot not found at $EMSDK_SYSROOT"
    exit 1
fi

echo "EMSDK sysroot: $EMSDK_SYSROOT"

# Tell bindgen to use Emscripten's headers instead of macOS headers
export BINDGEN_EXTRA_CLANG_ARGS="--target=wasm32-unknown-emscripten --sysroot=${EMSDK_SYSROOT}"

# Clean previous godot-ffi builds to force rebind
cargo clean -p godot-ffi 2>/dev/null || true

echo "Building for wasm32-unknown-emscripten..."
echo "BINDGEN_EXTRA_CLANG_ARGS: $BINDGEN_EXTRA_CLANG_ARGS"

# Build with nightly and build-std
cargo +nightly build -Zbuild-std --release --target wasm32-unknown-emscripten

echo "Build complete!"
