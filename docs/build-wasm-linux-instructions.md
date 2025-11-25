# Building WASM on Native Linux (Arch Linux)

This guide describes how to build the Godot-Rust WASM library on a native Linux system, avoiding the complexity of Docker and cross-compilation issues from macOS.

## Why Build on Linux?

Building WASM for gdext on macOS is problematic due to:
1. **Prebuilt bindings are 64-bit only** - WASM is 32-bit
2. **`api-custom` requires running Godot** - difficult in Docker on Apple Silicon
3. **Cross-compilation bindgen issues** - macOS headers leak into WASM builds

Building natively on Linux with `api-custom-json` avoids all these issues.

---

## Prerequisites

### Arch Linux

```bash
# Install base development tools
sudo pacman -S base-devel git wget

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install nightly toolchain and components
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
rustup +nightly target add wasm32-unknown-emscripten

# Install Emscripten
sudo pacman -S emscripten
# Or install from emsdk:
# git clone https://github.com/emscripten-core/emsdk.git ~/emsdk
# cd ~/emsdk && ./emsdk install 3.1.74 && ./emsdk activate 3.1.74
# source ~/emsdk/emsdk_env.sh

# Install LLVM/Clang for bindgen
sudo pacman -S llvm clang

# Install Godot (optional, only needed for api-custom, not api-custom-json)
sudo pacman -S godot
```

### Other Distros (Ubuntu/Debian)

```bash
# Install base tools
sudo apt-get update
sudo apt-get install -y build-essential git wget curl

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install nightly toolchain
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
rustup +nightly target add wasm32-unknown-emscripten

# Install Emscripten (recommend using emsdk for version control)
git clone https://github.com/emscripten-core/emsdk.git ~/emsdk
cd ~/emsdk
./emsdk install 3.1.74
./emsdk activate 3.1.74
source ~/emsdk/emsdk_env.sh

# Install LLVM/Clang
sudo apt-get install -y llvm clang libclang-dev

# Set LLVM_PATH (adjust version as needed)
export LLVM_PATH="/usr/lib/llvm-14"
```

---

## Build Steps

### Step 1: Clone the Repository

```bash
git clone https://github.com/softwarewrighter/godot-poc-rs.git
cd godot-poc-rs
```

### Step 2: Download extension_api.json

The `api-custom-json` feature requires the Godot extension API JSON file:

```bash
mkdir -p /tmp/godot-api
wget -q "https://raw.githubusercontent.com/godotengine/godot-cpp/godot-4.3-stable/gdextension/extension_api.json" \
    -O /tmp/godot-api/extension_api.json

# Verify download
ls -la /tmp/godot-api/extension_api.json
```

### Step 3: Set Environment Variables

```bash
# Point to the extension API JSON
export GODOT4_GDEXTENSION_JSON=/tmp/godot-api/extension_api.json

# Set LLVM path for bindgen (adjust path for your distro)
export LLVM_PATH="/usr/lib/llvm"  # Arch
# export LLVM_PATH="/usr/lib/llvm-14"  # Ubuntu/Debian

# Source Emscripten if using emsdk
source ~/emsdk/emsdk_env.sh
```

### Step 4: Modify Cargo.toml for WASM

Edit `Cargo.toml` to use `api-custom-json`:

```bash
# Backup original
cp Cargo.toml Cargo.toml.native

# Modify for WASM build
sed -i 's/features = \[.*\]/features = ["api-custom-json", "experimental-wasm", "experimental-wasm-nothreads", "lazy-function-tables"]/' Cargo.toml
```

Or manually edit `Cargo.toml`:

```toml
[dependencies.godot]
git = "https://github.com/godot-rust/gdext"
branch = "master"
features = ["api-custom-json", "experimental-wasm", "experimental-wasm-nothreads", "lazy-function-tables"]
```

### Step 5: Build for WASM

```bash
cargo +nightly build -Zbuild-std --release --target wasm32-unknown-emscripten
```

### Step 6: Verify Output

```bash
ls -la target/wasm32-unknown-emscripten/release/*.wasm
```

The output file should be at:
- `target/wasm32-unknown-emscripten/release/libgodot_poc_rs.wasm` or
- `target/wasm32-unknown-emscripten/release/godot_poc_rs.wasm`

---

## Copying WASM Back to macOS

After building on Linux, copy the `.wasm` file back to your macOS machine:

```bash
# On Linux, copy to a transfer location
scp target/wasm32-unknown-emscripten/release/*.wasm user@macos-machine:~/godot-poc-rs/target/wasm32-unknown-emscripten/release/

# Or use rsync
rsync -avz target/wasm32-unknown-emscripten/release/*.wasm user@macos-machine:~/godot-poc-rs/target/wasm32-unknown-emscripten/release/
```

Then on macOS, run the Godot export:

```bash
./scripts/export-to-web.sh --skip-rust
```

---

## Automated Build Script

Create this script on your Linux machine:

```bash
#!/bin/bash
# build-wasm-linux.sh

set -euo pipefail

# Configuration
EXTENSION_API_URL="https://raw.githubusercontent.com/godotengine/godot-cpp/godot-4.3-stable/gdextension/extension_api.json"
EXTENSION_API_PATH="/tmp/godot-api/extension_api.json"

echo "=== Godot-Rust WASM Build (Linux) ==="

# Download extension_api.json if not present
if [[ ! -f "$EXTENSION_API_PATH" ]]; then
    echo "Downloading extension_api.json..."
    mkdir -p "$(dirname "$EXTENSION_API_PATH")"
    wget -q "$EXTENSION_API_URL" -O "$EXTENSION_API_PATH"
fi

# Set environment
export GODOT4_GDEXTENSION_JSON="$EXTENSION_API_PATH"
export LLVM_PATH="${LLVM_PATH:-/usr/lib/llvm}"

# Source emsdk if available
if [[ -f "$HOME/emsdk/emsdk_env.sh" ]]; then
    source "$HOME/emsdk/emsdk_env.sh"
fi

# Verify emcc is available
if ! command -v emcc &> /dev/null; then
    echo "ERROR: emcc not found. Install Emscripten first."
    exit 1
fi

echo "Using Emscripten: $(emcc --version | head -1)"
echo "LLVM_PATH: $LLVM_PATH"
echo "GODOT4_GDEXTENSION_JSON: $GODOT4_GDEXTENSION_JSON"

# Backup and modify Cargo.toml
if [[ ! -f "Cargo.toml.native" ]]; then
    cp Cargo.toml Cargo.toml.native
fi
sed -i 's/features = \[.*\]/features = ["api-custom-json", "experimental-wasm", "experimental-wasm-nothreads", "lazy-function-tables"]/' Cargo.toml

# Build
echo "Building WASM..."
cargo +nightly build -Zbuild-std --release --target wasm32-unknown-emscripten

# Restore Cargo.toml
mv Cargo.toml.native Cargo.toml

echo ""
echo "=== Build Complete ==="
ls -la target/wasm32-unknown-emscripten/release/*.wasm 2>/dev/null || echo "WASM file location may vary"
```

---

## Troubleshooting

### "LLVM_PATH not set" or bindgen errors

```bash
# Find your LLVM installation
llvm-config --prefix

# Set LLVM_PATH accordingly
export LLVM_PATH="$(llvm-config --prefix)"
```

### "emcc not found"

```bash
# If using system Emscripten (Arch)
which emcc

# If using emsdk
source ~/emsdk/emsdk_env.sh
```

### JSON parse errors

Verify the extension_api.json downloaded correctly:

```bash
# Check file size (should be several MB)
ls -la /tmp/godot-api/extension_api.json

# Check it's valid JSON
head -c 100 /tmp/godot-api/extension_api.json
```

### Pointer size mismatch errors

If you still see pointer size errors, ensure you're using `api-custom-json` (not the default prebuilt bindings):

```bash
grep "features" Cargo.toml
# Should show: features = ["api-custom-json", ...]
```

---

## Alternative: Use GitHub Actions

Instead of building locally, you can use GitHub Actions with a Linux runner. See `.github/workflows/build-wasm.yml` (if present) or the example in `docs/godot-wasm-issues.md`.
