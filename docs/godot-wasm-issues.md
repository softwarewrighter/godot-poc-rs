# Godot-Rust WASM Export Issues and Solutions

## Overview

This document describes the challenges encountered when trying to export a Godot-Rust (gdext) project to WebAssembly from macOS, and the solutions/workarounds available.

---

## The Problem

When building for `wasm32-unknown-emscripten`, the build fails with size assertion errors:

```
error[E0080]: attempt to compute `12_usize - 24_usize`, which would overflow
  --> godot-ffi/.../gdextension_interface.rs:146:10
   |
146 |    [::std::mem::size_of::<__darwin_pthread_handler_rec>() - 24usize];
   |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

The errors all relate to darwin (macOS) pthread types having different sizes than expected:
- `__darwin_pthread_handler_rec`: 12 bytes (WASM) vs 24 bytes expected (macOS 64-bit)
- `_opaque_pthread_attr_t`: 60 bytes vs 64 bytes
- `_opaque_pthread_cond_t`: 44 bytes vs 48 bytes
- etc.

---

## Root Cause Analysis

### How gdext Bindings Work

1. **Prebuilt bindings**: gdext ships with prebuilt FFI bindings for Linux, macOS, and Windows in the `godot4-prebuilt` repository.

2. **Platform selection**: The `godot-bindings` crate selects the appropriate prebuilt bindings using Rust's `#[cfg]` attributes:
   ```rust
   #[cfg(target_os = "macos")]
   let s = include_str!("gdextension_interface_macos.rs");

   #[cfg(all(unix, not(target_os = "macos")))]
   let s = include_str!("gdextension_interface_linux.rs");
   ```

3. **Build scripts**: The `godot-bindings` crate is compiled as part of the build process via `build.rs`.

### Two Distinct Problems

#### Problem 1: macOS Host Cross-Compilation

When cross-compiling from macOS to WASM:

1. **Build scripts run on the host**: The `build.rs` scripts run on macOS (the host), not WASM (the target).

2. **`#[cfg]` evaluated at host compile time**: When `godot-bindings` is compiled on macOS, the `#[cfg(target_os = "macos")]` condition is true, so it selects the macOS prebuilt bindings.

3. **macOS bindings incompatible with WASM**: The macOS bindings contain darwin-specific pthread types that are 64-bit. Errors include `__darwin_pthread_handler_rec`, `_opaque_pthread_attr_t`, etc.

#### Problem 2: 64-bit vs 32-bit Pointer Sizes

Even when building from Linux (avoiding macOS bindings):

1. **All prebuilt bindings are 64-bit**: The Linux prebuilt bindings are generated for x86_64 Linux, using 8-byte pointers.

2. **WASM is 32-bit**: WebAssembly uses 4-byte pointers (wasm32).

3. **Struct sizes mismatch**: GDExtension structs contain pointers. A struct with 3 pointers is 24 bytes on 64-bit but only 12 bytes on 32-bit. Errors show: `GDExtensionInstanceBindingCallbacks: 12 bytes vs 24 expected`, `GDExtensionPropertyInfo: 24 bytes vs 48 expected`, etc.

### Why No Prebuilt Works

**From the official documentation**: "Prebuilts for wasm32 are not provided"

The `api-custom` feature is **required** for WASM builds - it regenerates bindings at build time using bindgen, properly targeting the 32-bit WASM platform.

---

## Attempted Solutions

### 1. Using `experimental-wasm` feature ❌

```toml
[dependencies.godot]
features = ["experimental-wasm", "lazy-function-tables"]
```

**Result**: Same error. The feature doesn't change how prebuilt bindings are selected.

### 2. Using `experimental-wasm-nothreads` feature ❌

```toml
[dependencies.godot]
features = ["experimental-wasm", "experimental-wasm-nothreads", "lazy-function-tables"]
```

**Result**: Same error. This feature only affects runtime threading behavior, not binding selection.

### 3. Setting `BINDGEN_EXTRA_CLANG_ARGS` ❌

```bash
export BINDGEN_EXTRA_CLANG_ARGS="--target=wasm32-unknown-emscripten --sysroot=/path/to/emsdk/sysroot"
```

**Result**: Doesn't help because prebuilt bindings are used, not regenerated via bindgen.

### 4. Using nightly with `-Zbuild-std` ❌

```bash
cargo +nightly build -Zbuild-std --target wasm32-unknown-emscripten
```

**Result**: Same error. The std rebuild doesn't affect how gdext selects prebuilt bindings.

### 5. Removing pthread flags from `.cargo/config.toml` ❌

```toml
[target.wasm32-unknown-emscripten]
rustflags = [
    "-C", "link-args=-sSIDE_MODULE=2",
    "-Zlink-native-libraries=no",
]
```

**Result**: Same error. The issue is in binding generation, not linking.

---

## Working Solutions

### Solution 1: Build on Linux with `api-custom` (Docker) ✅

The `api-custom` feature regenerates bindings using bindgen at build time, which properly targets the 32-bit WASM platform. This requires Godot to be installed.

**Dockerfile.wasm** (key parts):
```dockerfile
FROM rust:latest

# Install Godot (required for api-custom)
RUN wget https://github.com/godotengine/godot/releases/download/4.3-stable/Godot_v4.3-stable_linux.x86_64.zip \
    && unzip Godot_v4.3-stable_linux.x86_64.zip \
    && mv Godot_v4.3-stable_linux.x86_64 /usr/local/bin/godot

# Install Emscripten
RUN git clone https://github.com/emscripten-core/emsdk.git /opt/emsdk \
    && cd /opt/emsdk \
    && ./emsdk install 3.1.74 \
    && ./emsdk activate 3.1.74

# Install LLVM for bindgen
RUN apt-get install -y libllvm18 clang
ENV LLVM_PATH="/usr/lib/llvm-18"

# Install Rust nightly
RUN rustup toolchain install nightly \
    && rustup component add rust-src --toolchain nightly \
    && rustup +nightly target add wasm32-unknown-emscripten

# Use api-custom feature for WASM
# features = ["api-custom", "experimental-wasm", "experimental-wasm-nothreads", "lazy-function-tables"]

# Build
RUN cargo +nightly build -Zbuild-std --release --target wasm32-unknown-emscripten
```

**Usage**:
```bash
./scripts/build-wasm-docker.sh
```

### Solution 2: GitHub Actions (Linux Runner) ✅

Use a GitHub Actions workflow with a Linux runner and `api-custom`:

```yaml
jobs:
  build-wasm:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Godot
        run: |
          wget -q https://github.com/godotengine/godot/releases/download/4.3-stable/Godot_v4.3-stable_linux.x86_64.zip
          unzip -q Godot_v4.3-stable_linux.x86_64.zip
          sudo mv Godot_v4.3-stable_linux.x86_64 /usr/local/bin/godot

      - uses: mymindstorm/setup-emsdk@v14
        with:
          version: 3.1.74

      - name: Setup Rust
        run: |
          rustup toolchain install nightly
          rustup component add rust-src --toolchain nightly

      - name: Build WASM
        env:
          LLVM_PATH: /usr/lib/llvm-18
        run: |
          # Modify Cargo.toml for WASM build
          sed -i 's/features = \[.*\]/features = ["api-custom", "experimental-wasm", "experimental-wasm-nothreads", "lazy-function-tables"]/' Cargo.toml
          cargo +nightly build -Zbuild-std --release --target wasm32-unknown-emscripten
```

### Solution 3: Wait for Upstream Fix ⏳

The gdext project is aware of this issue (see [issue #1360](https://github.com/godot-rust/gdext/issues/1360)). A proper fix might involve:

1. Providing prebuilt 32-bit bindings for WASM
2. Better documentation about the `api-custom` requirement for WASM

---

## Related Links

- [gdext WebAssembly support issue #438](https://github.com/godot-rust/gdext/issues/438)
- [Windows cross-compilation issue #1360](https://github.com/godot-rust/gdext/issues/1360)
- [godot-rust web export documentation](https://godot-rust.github.io/book/toolchain/export-web.html)
- [Emscripten SDK](https://emscripten.org/docs/getting_started/downloads.html)

---

## Configuration Files

### Cargo.toml (for WASM builds with api-custom)

```toml
[dependencies.godot]
git = "https://github.com/godot-rust/gdext"
branch = "master"
features = ["api-custom", "experimental-wasm", "experimental-wasm-nothreads", "lazy-function-tables"]
```

### Cargo.toml (for native macOS/Linux/Windows builds)

```toml
[dependencies.godot]
git = "https://github.com/godot-rust/gdext"
branch = "master"
features = ["lazy-function-tables"]
```

### .cargo/config.toml

```toml
[target.wasm32-unknown-emscripten]
rustflags = [
    "-C", "link-args=-sSIDE_MODULE=2",
    "-Zlink-native-libraries=no",
    "-Cllvm-args=-enable-emscripten-cxx-exceptions=0",
]
```

### .gdextension (web paths)

```ini
[libraries]
web.debug.wasm32 = "res://../target/wasm32-unknown-emscripten/debug/godot_poc_rs.wasm"
web.release.wasm32 = "res://../target/wasm32-unknown-emscripten/release/godot_poc_rs.wasm"
```

---

## Summary

| Approach | Works | Notes |
|----------|-------|-------|
| Native build without `api-custom` | ❌ | Prebuilt bindings are 64-bit only |
| Native macOS with `api-custom` | ❌ | bindgen still targets host platform |
| Docker (Linux) with `api-custom` | ✅ | Requires Docker + Godot in container |
| GitHub Actions with `api-custom` | ✅ | Best for CI/CD |
| Upstream prebuilt 32-bit bindings | ⏳ | Not yet available |

### Key Insight

**The `api-custom` feature is REQUIRED for WASM builds.** This feature:
1. Requires Godot to be installed (to extract the API)
2. Uses bindgen to regenerate FFI bindings at build time
3. Properly targets the 32-bit WASM platform

Without `api-custom`, you're using 64-bit prebuilt bindings that are fundamentally incompatible with 32-bit WASM.

### Recommended Workflow

1. **Local development**: Use native macOS builds (fast iteration)
2. **WASM builds**: Use Docker or GitHub Actions with `api-custom` feature
3. **Keep separate Cargo.toml configs**: One for native, one for WASM (or use sed to modify)
