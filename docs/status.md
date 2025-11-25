# Project Status: Revolving Match-3 (Rust Edition)

## Current Status: Core Gameplay Complete, macOS Export Working

**Last Updated**: 2025-11-25

---

## Phase Progress

| Phase | Name | Status | Progress |
|-------|------|--------|----------|
| 1 | Rust & Godot Setup | 🟢 Complete | 100% |
| 2 | Core Grid System (Rust) | 🟢 Complete | 100% |
| 3 | Input & Selection (Rust) | 🟢 Complete | 100% |
| 4 | Match Detection (Rust) | 🟢 Complete | 100% |
| 5 | Gravity & Refill (Rust) | 🟢 Complete | 100% |
| 6 | Rotation Mechanic (Rust) | 🟢 Complete | 100% |
| 7 | Scoring System (Rust) | 🟢 Complete | 100% |
| 8 | Special Symbols (Rust) | ⚪ Not Started | 0% |
| 9 | Level System (Rust) | ⚪ Not Started | 0% |
| 10 | UI Integration | 🟡 Partial | 50% |
| 11 | Audio System (Rust) | ⚪ Not Started | 0% |
| 12 | Visual Polish | 🟡 Partial | 60% |
| 13 | macOS Export | 🟢 Complete | 100% |
| 14 | WebAssembly Build | 🔴 Blocked | 0% |
| 15 | Testing & Balance | ⚪ Not Started | 0% |

**Legend**: 🟢 Complete | 🟡 In Progress | ⚪ Not Started | 🔴 Blocked

---

## What's Working

### Core Gameplay ✅
- 8x8 game grid with colored symbols
- Click-to-swap adjacent symbols
- Match-3 detection (horizontal and vertical)
- Animated symbol clearing (shrink effect)
- Gravity drop animation
- Board refill with new symbols from top
- Cascade matching (chain reactions)
- Automatic rotation every 5 seconds
- 90-degree spin animation during rotation
- Deterministic color cycling (patterns consistent during rotation)
- Score tracking

### Export ✅
- macOS .app bundle export working
- Script: `./scripts/export-to-macos.sh`
- Output: `export/macos/Revolving Match-3 (Rust).app`

---

## Known Issues

### WASM/Web Export Blocked 🔴

The WebAssembly export is blocked due to a cross-compilation issue with `godot-ffi`:

**Problem**: When building for `wasm32-unknown-emscripten` on macOS, the `godot-ffi` crate generates FFI bindings at build-time using bindgen. However, bindgen generates bindings for the **host platform** (macOS 64-bit) instead of the **target platform** (WASM 32-bit). This causes size assertion failures:

```
error[E0080]: attempt to compute `12_usize - 24_usize`, which would overflow
  --> godot-ffi/.../gdextension_interface.rs:146:10
    |
146 |    [::std::mem::size_of::<__darwin_pthread_handler_rec>() - 24usize];
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

**Root Cause**: The darwin pthread types have different sizes on 64-bit (host) vs 32-bit (WASM target). The build.rs script runs on the host, generating incorrect size assertions for the target.

**Attempted Solutions**:
1. ❌ Using `experimental-wasm` feature - still generates host bindings
2. ❌ Using `experimental-wasm-nothreads` feature - same issue
3. ❌ Using nightly with `-Zbuild-std` - same issue
4. ❌ Removing pthread flags from `.cargo/config.toml` - same issue

**Potential Solutions to Try**:
1. Build on a Linux host (native 64-bit → WASM 32-bit may work better)
2. Use the `api-custom` feature with pre-generated WASM bindings
3. Wait for upstream gdext fix for macOS cross-compilation
4. Containerized build environment (Docker with Linux)

---

## Build Scripts

| Script | Purpose | Status |
|--------|---------|--------|
| `scripts/build.sh` | Build native Rust library | ✅ Working |
| `scripts/export-to-macos.sh` | Export macOS .app bundle | ✅ Working |
| `scripts/export-to-web.sh` | Export to WebAssembly | 🔴 Blocked |
| `scripts/serve-web.sh` | Local web server | ⚪ Untested |

---

## Project Structure

```
godot-poc-rs/
├── Cargo.toml              # Rust dependencies (gdext, rand)
├── .cargo/config.toml      # WASM build flags
├── src/
│   ├── lib.rs              # gdext entry point
│   ├── board.rs            # GameBoard (main game logic)
│   ├── symbols.rs          # Symbol, Grid, SymbolType
│   └── matching.rs         # Match detection logic
├── godot/
│   ├── project.godot       # Godot project config
│   ├── revolving_match3.gdextension
│   ├── scenes/
│   │   ├── main.tscn       # Main scene
│   │   └── game_board.tscn # Game board scene
│   └── src/main.gd         # GDScript entry
├── scripts/
│   ├── build.sh
│   ├── export-to-macos.sh
│   ├── export-to-web.sh
│   └── serve-web.sh
├── export/
│   └── macos/              # macOS export output
└── docs/
    ├── prd.md
    ├── architecture.md
    ├── design.md
    ├── plan.md
    └── status.md (this file)
```

---

## Development Environment

- **Rust**: 1.89.0 (edition 2024)
- **Godot**: 4.5.1.stable
- **gdext**: master branch (commit 6f85b4d5)
- **Platform**: macOS Darwin 24.6.0 (Apple Silicon)

---

## How to Build & Run

### Native Development
```bash
# Build Rust library
cargo build

# Open Godot editor
godot godot/project.godot
```

### macOS Export
```bash
# Full build and export
./scripts/export-to-macos.sh

# Export only (skip Rust build)
./scripts/export-to-macos.sh --skip-rust

# Run the app
open export/macos/Revolving\ Match-3\ \(Rust\).app
```

---

## Next Steps

1. **Consider alternative web deployment strategies**:
   - Build on Linux (native or Docker)
   - Use GitHub Actions for WASM builds
   - Wait for gdext WASM improvements

2. **Continue feature development** (can test natively):
   - Special symbols (wild, bomb, etc.)
   - Level progression system
   - Audio integration
   - UI polish

---

## Quick Links

- [PRD](./prd.md) - Product requirements
- [Architecture](./architecture.md) - Technical architecture
- [Design](./design.md) - Game design document
- [Plan](./plan.md) - Development plan
