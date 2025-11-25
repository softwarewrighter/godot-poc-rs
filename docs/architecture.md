# System Architecture: Revolving Match-3 (Rust Edition)

## Overview

This document outlines the technical architecture for the Revolving Match-3 game built with Godot 4.x and **Rust via gdext (godot-rust)**, targeting WebAssembly for web deployment.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        Game Application                              │
├─────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────┐  │
│  │  Godot Scenes   │  │   Rust Logic    │  │     Resources       │  │
│  │    (tscn)       │  │   (gdext/wasm)  │  │                     │  │
│  ├─────────────────┤  ├─────────────────┤  ├─────────────────────┤  │
│  │ Main Menu       │  │ GameBoard       │  │ Symbol Definitions  │  │
│  │ Game Board      │  │ MatchFinder     │  │ Level Data          │  │
│  │ UI Overlay      │  │ Grid            │  │ Audio Assets        │  │
│  │ Settings        │  │ SymbolManager   │  │ Visual Assets       │  │
│  │ Level Select    │  │ ScoreSystem     │  │ Configuration       │  │
│  │                 │  │ RotationSystem  │  │                     │  │
│  └─────────────────┘  └─────────────────┘  └─────────────────────┘  │
├─────────────────────────────────────────────────────────────────────┤
│                      Godot Engine 4.x + gdext                        │
├─────────────────────────────────────────────────────────────────────┤
│                   WebAssembly Runtime (Browser)                      │
└─────────────────────────────────────────────────────────────────────┘
```

## Rust-Godot Integration (gdext)

### gdext Overview
The project uses [godot-rust/gdext](https://github.com/godot-rust/gdext) to write game logic in Rust that integrates with Godot 4.x.

```rust
// Example: Rust class exposed to Godot
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct GameBoard {
    base: Base<Node2D>,
    grid: Grid,
    symbols: Vec<Symbol>,
    score: i32,
}

#[godot_api]
impl INode2D for GameBoard {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            grid: Grid::new(8, 8),
            symbols: Vec::new(),
            score: 0,
        }
    }

    fn ready(&mut self) {
        self.initialize_board();
    }

    fn process(&mut self, delta: f64) {
        self.update_rotation(delta);
    }
}
```

### Library Structure
```rust
// src/lib.rs - Entry point for gdext
use godot::prelude::*;

mod board;
mod symbols;
mod matching;
mod scoring;
mod rotation;
mod autoload;

struct RevolvingMatch3;

#[gdextension]
unsafe impl ExtensionLibrary for RevolvingMatch3 {}
```

## Directory Structure

```
godot-poc-rs/
├── Cargo.toml                 # Rust project configuration
├── rust-toolchain.toml        # Rust version pinning
├── .cargo/
│   └── config.toml            # Cargo configuration (wasm target)
│
├── src/                       # Rust source code
│   ├── lib.rs                # gdext entry point
│   ├── board/
│   │   ├── mod.rs
│   │   ├── game_board.rs     # Main board logic
│   │   ├── grid.rs           # Grid data structure
│   │   └── cell.rs           # Cell representation
│   ├── symbols/
│   │   ├── mod.rs
│   │   ├── symbol.rs         # Symbol struct and logic
│   │   ├── symbol_type.rs    # Symbol type enum
│   │   └── factory.rs        # Symbol creation
│   ├── matching/
│   │   ├── mod.rs
│   │   ├── finder.rs         # Match detection algorithm
│   │   └── match_result.rs   # Match result struct
│   ├── scoring/
│   │   ├── mod.rs
│   │   └── score_manager.rs  # Scoring logic
│   ├── rotation/
│   │   ├── mod.rs
│   │   └── rotation_manager.rs # Rotation timing
│   └── autoload/
│       ├── mod.rs
│       ├── game_manager.rs   # Global game state
│       └── audio_manager.rs  # Audio handling
│
├── godot/                     # Godot project files
│   ├── project.godot         # Godot project configuration
│   ├── export_presets.cfg    # Export configurations
│   ├── scenes/
│   │   ├── main.tscn         # Main scene
│   │   ├── game/
│   │   │   ├── game_board.tscn
│   │   │   └── symbol.tscn
│   │   └── ui/
│   │       ├── main_menu.tscn
│   │       ├── hud.tscn
│   │       └── pause_menu.tscn
│   ├── assets/
│   │   ├── sprites/
│   │   ├── audio/
│   │   └── fonts/
│   └── resources/
│       ├── themes/
│       ├── levels/
│       └── symbols/
│
├── scripts/                   # Build and deployment scripts
│   ├── build.sh              # Build Rust library
│   ├── export-to-web.sh      # Export to WebAssembly
│   └── serve-web.sh          # Local web server
│
├── docs/                      # Documentation
│   ├── prd.md
│   ├── architecture.md
│   ├── design.md
│   ├── plan.md
│   └── status.md
│
└── export/                    # Build outputs
    └── web/                   # Web export files
```

## Core Rust Systems

### 1. GameBoard

Central gameplay system managing the grid and game state:

```rust
pub struct GameBoard {
    grid: Grid,
    symbols: Vec<Symbol>,
    selected_symbol: Option<usize>,
    score: i32,
    rotation_timer: f64,
    state: GameState,
}

impl GameBoard {
    pub fn initialize(&mut self, level_data: &LevelData);
    pub fn handle_input(&mut self, position: Vector2);
    pub fn swap_symbols(&mut self, pos1: Vector2i, pos2: Vector2i);
    pub fn check_matches(&self) -> Vec<Match>;
    pub fn clear_matches(&mut self, matches: &[Match]);
    pub fn apply_gravity(&mut self);
    pub fn spawn_new_symbols(&mut self);
    pub fn trigger_rotation(&mut self);
}
```

### 2. Grid

Data structure for the game board:

```rust
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<Option<Symbol>>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self;
    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Symbol>;
    pub fn set_cell(&mut self, x: usize, y: usize, symbol: Option<Symbol>);
    pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)>;
    pub fn is_valid_position(&self, x: i32, y: i32) -> bool;
}
```

### 3. MatchFinder

Algorithm for detecting matches:

```rust
pub struct MatchFinder;

impl MatchFinder {
    pub fn find_horizontal_matches(grid: &Grid) -> Vec<Match>;
    pub fn find_vertical_matches(grid: &Grid) -> Vec<Match>;
    pub fn find_all_matches(grid: &Grid) -> Vec<Match>;
    pub fn merge_overlapping_matches(matches: Vec<Match>) -> Vec<Match>;
}
```

### 4. Symbol

Symbol representation with rotation state:

```rust
#[derive(Clone)]
pub struct Symbol {
    symbol_type: SymbolType,
    rotation_state: u8,
    faces: [SymbolType; 4],
    grid_position: Vector2i,
}

impl Symbol {
    pub fn rotate(&mut self);
    pub fn get_current_face(&self) -> SymbolType;
    pub fn matches_with(&self, other: &Symbol) -> bool;
}
```

### 5. RotationManager

Handles the revolving mechanic:

```rust
pub struct RotationManager {
    interval: f64,
    elapsed: f64,
    is_rotating: bool,
}

impl RotationManager {
    pub fn update(&mut self, delta: f64) -> bool; // Returns true when rotation triggers
    pub fn reset(&mut self);
    pub fn set_interval(&mut self, interval: f64);
}
```

## WebAssembly Build Pipeline

### Compilation Flow
```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   Rust Source   │────▶│   cargo build   │────▶│   .wasm + .so   │
│   (src/*.rs)    │     │   --target      │     │   library       │
└─────────────────┘     │   wasm32        │     └────────┬────────┘
                        └─────────────────┘              │
                                                         ▼
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   Godot Export  │◀────│   Godot Editor  │◀────│   .gdextension  │
│   (web build)   │     │   (loads lib)   │     │   config        │
└────────┬────────┘     └─────────────────┘     └─────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Web Export Output                            │
│  index.html, index.js, index.wasm, index.pck, service worker    │
└─────────────────────────────────────────────────────────────────┘
```

### Target Configuration
```toml
# .cargo/config.toml
[target.wasm32-unknown-emscripten]
rustflags = ["-C", "link-arg=-sSIDE_MODULE=2"]

[build]
target = "wasm32-unknown-emscripten"
```

### GDExtension Configuration
```toml
# godot/revolving_match3.gdextension
[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.2

[libraries]
macos.debug = "res://../target/debug/librevolving_match3.dylib"
macos.release = "res://../target/release/librevolving_match3.dylib"
linux.debug.x86_64 = "res://../target/debug/librevolving_match3.so"
linux.release.x86_64 = "res://../target/release/librevolving_match3.so"
windows.debug.x86_64 = "res://../target/debug/revolving_match3.dll"
windows.release.x86_64 = "res://../target/release/revolving_match3.dll"
web.debug.wasm32 = "res://../target/wasm32-unknown-emscripten/debug/revolving_match3.wasm"
web.release.wasm32 = "res://../target/wasm32-unknown-emscripten/release/revolving_match3.wasm"
```

## Signal Architecture

Rust classes emit Godot signals for decoupled communication:

```rust
#[godot_api]
impl GameBoard {
    #[signal]
    fn symbol_selected(position: Vector2i);

    #[signal]
    fn symbols_swapped(pos1: Vector2i, pos2: Vector2i);

    #[signal]
    fn matches_found(count: i32);

    #[signal]
    fn score_changed(new_score: i32);

    #[signal]
    fn rotation_started();

    #[signal]
    fn rotation_completed();
}
```

## Data Flow

### Game Loop
```
┌──────────────┐
│  Player      │
│  Input       │
└──────┬───────┘
       │
       ▼
┌──────────────┐     ┌──────────────┐
│  Validate    │────▶│  Execute     │
│  Move (Rust) │     │  Swap (Rust) │
└──────────────┘     └──────┬───────┘
                            │
       ┌────────────────────┘
       ▼
┌──────────────┐     ┌──────────────┐
│  Check       │────▶│  Clear       │
│  Matches     │     │  Matches     │
│  (Rust)      │     │  (Rust)      │
└──────────────┘     └──────┬───────┘
                            │
       ┌────────────────────┘
       ▼
┌──────────────┐     ┌──────────────┐
│  Apply       │────▶│  Spawn New   │
│  Gravity     │     │  Symbols     │
│  (Rust)      │     │  (Rust)      │
└──────────────┘     └──────┬───────┘
                            │
       ┌────────────────────┘
       ▼
┌──────────────┐     ┌──────────────┐
│  Check       │────▶│  Update      │
│  Cascades    │     │  Score       │
│  (Rust)      │     │  (Rust)      │
└──────────────┘     └──────────────┘
```

## Performance Considerations

1. **Zero-Copy Where Possible**: Pass references to Godot nodes
2. **Batch Operations**: Process matches in bulk
3. **Efficient Data Structures**: Use Rust's Vec and HashMap
4. **WASM Optimization**: Build with `--release` and LTO
5. **Object Pooling**: Reuse symbol instances

## Testing Strategy

### Rust Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_match() {
        let mut grid = Grid::new(8, 8);
        // Setup and test match detection
    }

    #[test]
    fn test_gravity() {
        // Test symbol falling
    }
}
```

### Integration Testing
- Manual playtesting in Godot editor
- Web browser testing
- Cross-browser compatibility checks
