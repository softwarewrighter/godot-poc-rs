# Development Plan: Revolving Match-3 (Rust Edition)

## Development Phases

### Phase 1: Rust & Godot Setup
**Goal**: Configure Rust project with gdext for Godot 4.x integration

#### Tasks
- [ ] Configure Cargo.toml with gdext dependencies
- [ ] Set up rust-toolchain.toml for consistent builds
- [ ] Create .cargo/config.toml for wasm target
- [ ] Create basic lib.rs with gdext entry point
- [ ] Set up Godot project structure (godot/ directory)
- [ ] Create .gdextension configuration file
- [ ] Verify Rust library loads in Godot editor
- [ ] Create placeholder main scene

#### Deliverable
Rust library compiles and loads in Godot. A simple Rust class is visible in Godot editor.

---

### Phase 2: Core Grid System (Rust)
**Goal**: Implement game board with Rust

#### Tasks
- [ ] Create Grid struct in Rust
- [ ] Implement Symbol struct with rotation state
- [ ] Create SymbolType enum
- [ ] Implement GameBoard as GodotClass
- [ ] Create symbol spawning logic in Rust
- [ ] Connect Rust GameBoard to Godot scene
- [ ] Display 8x8 grid with placeholder visuals

#### Deliverable
A visible 8x8 grid populated with random symbols, all logic in Rust.

---

### Phase 3: Input & Selection (Rust)
**Goal**: Handle player input through Rust

#### Tasks
- [ ] Implement click detection in Rust GameBoard
- [ ] Add symbol selection logic
- [ ] Implement adjacent cell detection
- [ ] Create swap input handling
- [ ] Add visual selection feedback via signals
- [ ] Implement swap animation coordination
- [ ] Handle invalid selections

#### Deliverable
Player can select symbols and swap adjacent symbols, with Rust handling all logic.

---

### Phase 4: Match Detection (Rust)
**Goal**: Implement match-finding algorithm in Rust

#### Tasks
- [ ] Create MatchFinder struct
- [ ] Implement horizontal match detection
- [ ] Implement vertical match detection
- [ ] Add match merging for overlaps
- [ ] Create match validation for swaps
- [ ] Implement revert for invalid swaps
- [ ] Add match clearing logic
- [ ] Emit signals for visual effects

#### Deliverable
Swapping creates matches, invalid swaps revert. All match logic in pure Rust.

---

### Phase 5: Gravity & Refill (Rust)
**Goal**: Implement board refill in Rust

#### Tasks
- [ ] Implement gravity algorithm
- [ ] Create falling animation coordination
- [ ] Add new symbol spawning
- [ ] Implement cascade detection
- [ ] Handle multiple cascades
- [ ] Emit signals for particle effects

#### Deliverable
After matches clear, symbols fall and new ones spawn. Cascades work correctly.

---

### Phase 6: Rotation Mechanic (Rust)
**Goal**: Implement revolving symbols in Rust

#### Tasks
- [ ] Create RotationManager struct
- [ ] Implement rotation timer
- [ ] Add multi-face symbol state
- [ ] Create rotation animation coordination
- [ ] Update match detection for current face
- [ ] Add rotation UI indicator signal
- [ ] Test matches across rotation

#### Deliverable
Symbols rotate periodically, changing their matchable face.

---

### Phase 7: Scoring System (Rust)
**Goal**: Track and calculate scores in Rust

#### Tasks
- [ ] Create ScoreManager struct
- [ ] Implement base scoring for matches
- [ ] Add combo multiplier system
- [ ] Create rotation bonus scoring
- [ ] Emit score update signals
- [ ] Implement high score tracking
- [ ] Add score persistence (save/load)

#### Deliverable
Scores are calculated in Rust, displayed via signals.

---

### Phase 8: Special Symbols (Rust)
**Goal**: Create special symbols from large matches

#### Tasks
- [ ] Define special symbol types in Rust enum
- [ ] Implement 4-match -> Line Blast
- [ ] Implement 5-match -> special symbols
- [ ] Create special symbol activation logic
- [ ] Emit signals for special effects
- [ ] Add special symbol sounds

#### Deliverable
Large matches create special symbols with unique effects.

---

### Phase 9: Level System (Rust)
**Goal**: Implement level progression in Rust

#### Tasks
- [ ] Create LevelData struct in Rust
- [ ] Design initial level set (10-20 levels)
- [ ] Implement objective system
- [ ] Add move/time limit tracking
- [ ] Create level complete/fail conditions
- [ ] Implement star rating calculation
- [ ] Add level data loading from resources
- [ ] Create level transition signals

#### Deliverable
Levels load from data, objectives tracked in Rust.

---

### Phase 10: UI Integration
**Goal**: Connect Rust logic to Godot UI

#### Tasks
- [ ] Create main menu scene
- [ ] Implement settings menu
- [ ] Design in-game HUD
- [ ] Add pause menu
- [ ] Create level complete popup
- [ ] Add level fail popup
- [ ] Implement level selection screen
- [ ] Connect all UI to Rust signals

#### Deliverable
All menus and UI elements functional, driven by Rust signals.

---

### Phase 11: Audio System (Rust)
**Goal**: Implement audio management in Rust

#### Tasks
- [ ] Create AudioManager as autoload
- [ ] Source/create background music
- [ ] Source/create sound effects
- [ ] Implement music playback control
- [ ] Add SFX playback via signals
- [ ] Implement volume controls
- [ ] Add audio feedback layering

#### Deliverable
Full audio experience with Rust-controlled playback.

---

### Phase 12: Visual Polish
**Goal**: Enhance visuals

#### Tasks
- [ ] Create final symbol art
- [ ] Design backgrounds
- [ ] Add particle effects (Godot)
- [ ] Implement screen shake
- [ ] Polish all animations
- [ ] Add visual juice elements

#### Deliverable
Visually polished game.

---

### Phase 13: WebAssembly Build
**Goal**: Configure and test WASM export

#### Tasks
- [ ] Configure wasm32-unknown-emscripten target
- [ ] Set up Emscripten SDK
- [ ] Build Rust library for WASM
- [ ] Configure Godot web export preset
- [ ] Test web build locally
- [ ] Fix any WASM-specific issues
- [ ] Optimize bundle size
- [ ] Test cross-browser compatibility

#### Deliverable
Game runs in web browser via WebAssembly.

---

### Phase 14: Testing & Balance
**Goal**: Ensure quality and balanced gameplay

#### Tasks
- [ ] Write Rust unit tests for core logic
- [ ] Playtest all levels
- [ ] Balance difficulty curve
- [ ] Fix discovered bugs
- [ ] Performance optimization
- [ ] Test on multiple browsers
- [ ] Memory profiling

#### Deliverable
Stable, balanced, bug-free game.

---

### Phase 15: Web Deployment
**Goal**: Deploy to web

#### Tasks
- [ ] Configure PWA manifest
- [ ] Set up service worker
- [ ] Configure COOP/COEP headers
- [ ] Create deployment scripts
- [ ] Test offline functionality
- [ ] Prepare hosting (Vercel/Netlify)
- [ ] Create store assets (icons, screenshots)

#### Deliverable
Web-deployed game accessible via URL.

---

## Milestone Summary

| Milestone | Phases | Description |
|-----------|--------|-------------|
| M1: Rust Foundation | 1-2 | Rust-Godot integration working |
| M2: Core Gameplay | 3-6 | Full match-3 with rotation in Rust |
| M3: Complete Game | 7-11 | Levels, scoring, UI, audio |
| M4: Web Ready | 12-13 | Visual polish and WASM build |
| M5: Release | 14-15 | Testing and web deployment |

## Technical Notes

### Development Environment
- Rust (stable, specified in rust-toolchain.toml)
- Godot Engine 4.x (latest stable)
- gdext (godot-rust) for Rust integration
- Emscripten SDK for WASM compilation
- Git for version control

### Build Commands
```bash
# Development build (native)
./scripts/build.sh

# Web export
./scripts/export-to-web.sh

# Local web server
./scripts/serve-web.sh
```

### Testing Approach
- Rust unit tests: `cargo test`
- Integration testing in Godot editor
- Web testing in multiple browsers
- Performance profiling with browser dev tools

### Recommended Order
Follow phases sequentially. Rust integration (Phase 1) is critical - ensure it works before proceeding. Web build (Phase 13) can be tested earlier if desired.
