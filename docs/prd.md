# Product Requirements Document: Revolving Match-3 (Rust Edition)

## Overview

A match-3 puzzle game built with Godot Engine and **Rust via gdext (godot-rust)**, featuring a unique revolving symbols mechanic. This is the Rust implementation of the original GDScript-based godot-poc project.

## Product Vision

Create an engaging match-3 game where symbols rotate and revolve, adding strategic depth to traditional match-3 gameplay. This version prioritizes **Rust** for all game logic, demonstrating the viability of Rust for web-deployed Godot games.

## Technology Stack

### Primary Technologies
- **Engine**: Godot 4.x
- **Game Logic**: Rust (via godot-rust/gdext)
- **Build Target**: WebAssembly (WASM) for web deployment
- **Scripting Fallback**: GDScript only where Rust integration is not possible

### Why Rust?
- Type safety and memory safety without garbage collection
- Performance comparable to native code
- WebAssembly compilation for web deployment
- Strong ecosystem (cargo, crates.io)
- Better maintainability for complex game logic

## Target Audience

- Casual puzzle game enthusiasts
- Players who enjoy match-3 games with unique mechanics
- Ages 10+

## Core Features

### 1. Revolving Symbols Mechanic
- Symbols rotate on the game board at defined intervals
- Rotation reveals different symbol faces
- Strategic timing of matches based on symbol rotation state

### 2. Match-3 Core Gameplay
- Grid-based game board (8x8 standard)
- Swap adjacent symbols to create matches of 3+
- Matches clear symbols and award points
- Gravity causes symbols to fall and fill gaps
- New symbols spawn from the top

### 3. Symbol Types
- 6 basic symbol types with distinct colors
- Each symbol can have multiple "faces" that rotate
- Special symbols (bombs, line clears) from larger matches

### 4. Scoring System
- Points for basic matches (3, 4, 5+ in a row)
- Combo multipliers for chain reactions
- Bonus points for timing matches with rotation states

### 5. Level Progression
- Multiple levels with increasing difficulty
- Level objectives (score targets, specific symbol clears)
- Star rating system (1-3 stars)

## Technical Requirements

### Platform
- **Primary**: Web (WebAssembly)
- **Secondary**: Desktop (Windows, macOS, Linux)
- **Engine**: Godot 4.x with gdext

### Performance
- 60 FPS target
- Smooth animations for symbol rotation and matching
- Responsive input handling
- Efficient WASM bundle size

### Web Deployment
- Progressive Web App (PWA) capable
- Proper COOP/COEP headers for SharedArrayBuffer support
- Service worker for offline play
- Cross-browser compatibility

### Audio
- Background music
- Sound effects for matches, rotations, and special events
- Volume controls

### Visual
- Clean, colorful 2D graphics
- Smooth rotation animations
- Particle effects for matches and combos
- UI scaling for different resolutions

## Rust Implementation Goals

### What Should Be in Rust
- Core game logic (match detection, gravity, scoring)
- Symbol state management
- Rotation system
- Level data and objectives
- Save/load functionality
- Input processing logic

### What May Remain in GDScript
- Scene tree manipulation (if needed)
- Some UI bindings (if simpler)
- Quick prototyping elements

## Success Metrics

- Smooth, bug-free gameplay
- Intuitive controls and mechanics
- Successful web deployment via WASM
- Rust code coverage > 80% of game logic
- Polished visual and audio presentation

## Out of Scope (Initial Release)

- Multiplayer functionality
- Mobile platform support
- In-app purchases
- Cloud save synchronization
- Achievements system

## Future Considerations

- Mobile port
- Additional game modes
- Social features
- Daily challenges
- Full Rust implementation (100%)
