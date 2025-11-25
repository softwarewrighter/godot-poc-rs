# Game Design Document: Revolving Match-3 (Rust Edition)

## Game Concept

### Elevator Pitch
A match-3 puzzle game where symbols continuously revolve, revealing different faces that players must strategically match before they rotate again. Built with **Rust** for maximum performance and web deployment.

### Core Innovation
Unlike traditional match-3 games where symbols are static, each symbol in Revolving Match-3 is a multi-faced piece that rotates on a timer. Players must time their matches based on what faces are currently showing and anticipate upcoming rotations.

### Rust Advantage
Using Rust for the game logic provides:
- Predictable performance with no garbage collection pauses
- Memory safety guarantees
- Easy WebAssembly compilation for web deployment
- Strong typing catches bugs at compile time

## Gameplay Mechanics

### Basic Match-3 Rules

1. **Grid Layout**: 8x8 grid of symbols
2. **Swapping**: Click one symbol, then click an adjacent symbol to swap
3. **Valid Moves**: Swaps are only valid if they create a match of 3+ symbols
4. **Matching**: 3 or more identical symbols in a row (horizontal or vertical)
5. **Clearing**: Matched symbols are removed from the board
6. **Gravity**: Symbols above cleared spaces fall down
7. **Refill**: New symbols spawn from the top to fill empty spaces

### Revolving Mechanic

#### Symbol Faces
Each symbol has 4 faces (like a rotating cube viewed from above):
- Face 1: Primary symbol (e.g., Red Gem)
- Face 2: Secondary symbol (e.g., Blue Gem)
- Face 3: Tertiary symbol (e.g., Green Gem)
- Face 4: Quaternary symbol (e.g., Yellow Gem)

#### Rotation Behavior
- **Rotation Interval**: Symbols rotate every 5 seconds (configurable per level)
- **Rotation Direction**: All symbols rotate in the same direction (clockwise)
- **Visual Indicator**:
  - Subtle rotation animation shows what's coming
  - Timer bar shows time until next rotation
- **Rotation Sync**: All symbols rotate simultaneously

#### Strategic Implications
- Plan matches based on current AND upcoming faces
- Create setups that will result in matches after rotation
- Combo potential: Make a match just before rotation triggers chain reaction

### Symbol Types

#### Basic Symbols (6 types)
| Symbol | Color | Rust Enum |
|--------|-------|-----------|
| Gem A | Red | `SymbolType::Red` |
| Gem B | Blue | `SymbolType::Blue` |
| Gem C | Green | `SymbolType::Green` |
| Gem D | Yellow | `SymbolType::Yellow` |
| Gem E | Purple | `SymbolType::Purple` |
| Gem F | Orange | `SymbolType::Orange` |

#### Rust Implementation
```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolType {
    Red,
    Blue,
    Green,
    Yellow,
    Purple,
    Orange,
}

impl SymbolType {
    pub fn color(&self) -> Color {
        match self {
            Self::Red => Color::from_rgb(1.0, 0.2, 0.2),
            Self::Blue => Color::from_rgb(0.2, 0.4, 1.0),
            Self::Green => Color::from_rgb(0.2, 0.8, 0.2),
            Self::Yellow => Color::from_rgb(1.0, 0.9, 0.2),
            Self::Purple => Color::from_rgb(0.6, 0.2, 0.8),
            Self::Orange => Color::from_rgb(1.0, 0.5, 0.1),
        }
    }
}
```

#### Face Combinations
Each physical symbol cycles through 4 different basic symbol types:
- Example: A symbol might cycle: Red -> Blue -> Green -> Yellow -> Red...

#### Special Symbols (Created by matches)

| Match Type | Special Created | Effect |
|------------|-----------------|--------|
| 4 in a row | Line Blast | Clears entire row or column |
| 5 in a row | Rotation Bomb | Rotates all symbols 1 step |
| L-shape (5) | Cross Blast | Clears row AND column |
| T-shape (5) | Color Bomb | Clears all of one symbol type |
| 6+ in a row | Super Bomb | Clears 3x3 area |

### Scoring System

#### Base Points
| Match Size | Points |
|------------|--------|
| 3 symbols | 50 |
| 4 symbols | 100 |
| 5 symbols | 200 |
| 6+ symbols | 400 |

#### Multipliers
- **Combo Multiplier**: Each cascade adds +1 to multiplier (max 10x)
- **Rotation Match Bonus**: +50% if match happens within 1 second of rotation
- **Special Symbol Bonus**: 2x points for special symbol activations

#### Rust Scoring Implementation
```rust
pub struct ScoreManager {
    score: i32,
    combo_multiplier: i32,
}

impl ScoreManager {
    pub fn calculate_match_score(&self, match_size: usize) -> i32 {
        let base = match match_size {
            3 => 50,
            4 => 100,
            5 => 200,
            _ => 400,
        };
        base * self.combo_multiplier
    }

    pub fn increment_combo(&mut self) {
        self.combo_multiplier = (self.combo_multiplier + 1).min(10);
    }

    pub fn reset_combo(&mut self) {
        self.combo_multiplier = 1;
    }
}
```

## Level Design

### Level Structure

#### Level Components
1. **Grid Configuration**: Size and shape of playable area
2. **Available Symbols**: Which symbol face combinations appear
3. **Rotation Speed**: Time between rotations
4. **Objectives**: What player must achieve
5. **Constraints**: Move limit, time limit, or unlimited

#### Objective Types
| Objective | Description |
|-----------|-------------|
| Score Target | Reach X points |
| Clear Symbols | Clear X symbols of specific type |
| Clear All | Remove all of certain symbols from board |
| Chain Reaction | Create cascade of X+ matches |
| Special Symbols | Create X special symbols |

### Level Progression

#### Difficulty Curve
- **Levels 1-10**: Tutorial, 4 symbol types, slow rotation (7s)
- **Levels 11-25**: Standard, 5 symbol types, medium rotation (5s)
- **Levels 26-50**: Challenging, 6 symbol types, fast rotation (4s)
- **Levels 51+**: Expert, complex objectives, variable rotation

#### Star Rating
| Stars | Requirement |
|-------|-------------|
| 1 Star | Complete objective |
| 2 Stars | Complete with 20%+ remaining moves/time |
| 3 Stars | Complete with 40%+ remaining moves/time AND hit score target |

### Tutorial Levels

**Level 1: "First Match"**
- Objective: Match 3 symbols
- Grid: 5x5
- No rotation (disabled)
- Teaches: Basic matching

**Level 2: "Bigger is Better"**
- Objective: Match 4 symbols
- Grid: 6x6
- No rotation
- Teaches: 4-match creates special

**Level 3: "Everything Changes"**
- Objective: Score 500 points
- Grid: 6x6
- Slow rotation (10s)
- Teaches: Rotation mechanic

**Level 4: "Plan Ahead"**
- Objective: Create a cascade
- Grid: 7x7
- Medium rotation (7s)
- Teaches: Planning for rotation

## User Interface

### Main Menu
```
┌─────────────────────────────┐
│     REVOLVING MATCH-3       │
│       (Rust Edition)        │
│                             │
│        [PLAY]               │
│                             │
│      [LEVEL SELECT]         │
│                             │
│       [SETTINGS]            │
│                             │
│         [QUIT]              │
└─────────────────────────────┘
```

### Game HUD
```
┌─────────────────────────────────────┐
│ Score: 12,450    ★★☆    Moves: 15   │
├─────────────────────────────────────┤
│                                     │
│   ┌─────────────────────────┐       │
│   │                         │       │
│   │      GAME BOARD         │       │
│   │         8x8             │       │
│   │                         │       │
│   └─────────────────────────┘       │
│                                     │
│ [Rotation Timer Bar ████████░░]     │
│                                     │
│ Objective: Clear 20 Red Gems (15/20)│
├─────────────────────────────────────┤
│  [PAUSE]              [HINT]        │
└─────────────────────────────────────┘
```

### Visual Feedback

#### Symbol Selection
- Selected symbol: Glowing outline, slight scale up
- Valid swap targets: Subtle highlight

#### Matching
- Match found: Symbols flash white
- Clearing: Symbols shrink and fade with particles
- Points popup: Float up from cleared area

#### Rotation
- Pre-rotation: Symbols wobble slightly
- During rotation: 3D flip animation
- Post-rotation: Brief settle animation

#### Cascades
- Screen shake (subtle) for large cascades
- Combo counter appears and grows

## Audio Design

### Music
- **Menu**: Calm, inviting ambient track
- **Gameplay**: Upbeat, moderately paced puzzle music
- **Intensity**: Music layers add with combo multiplier
- **Victory**: Celebratory fanfare
- **Failure**: Sympathetic, encouraging tone

### Sound Effects

| Event | Sound Description |
|-------|-------------------|
| Symbol select | Soft click/pop |
| Invalid move | Dull thud |
| Swap | Whoosh |
| Match 3 | Bright chime |
| Match 4+ | Ascending chime scale |
| Symbol clear | Sparkle/shatter |
| Cascade | Building intensity sounds |
| Rotation start | Mechanical whir |
| Rotation complete | Click into place |
| Special symbol create | Power-up sound |
| Special symbol activate | Explosion/energy burst |
| Level complete | Victory jingle |
| Level fail | Gentle failure sound |

## Visual Style

### Art Direction
- **Style**: Clean, modern, slightly glossy 2D
- **Color Palette**: Vibrant, high contrast colors
- **Symbols**: Geometric shapes with subtle gradients
- **Background**: Soft, non-distracting patterns
- **UI**: Minimalist, rounded corners, drop shadows

### Animation Principles
- **Snappy**: Quick response to inputs
- **Juicy**: Satisfying feedback for actions
- **Smooth**: 60 FPS target, eased transitions
- **Clear**: Animations communicate game state

### Symbol Visual Design
Each symbol face should be:
- Instantly recognizable by shape AND color
- Distinct from all other symbol types
- Readable at small sizes
- Visually appealing when rotating

## Game Feel

### Responsiveness
- Input -> Visual response: < 50ms
- Swap animation: 150ms
- Match clear: 200ms
- Gravity fall: 100ms per cell

### Feedback Loops
1. **Immediate**: Visual/audio response to every action
2. **Short-term**: Combos, cascades, score popups
3. **Medium-term**: Level completion, star ratings
4. **Long-term**: Level progression, unlocks

### Player Agency
- Moves should feel intentional, not random
- Rotation adds planning, not frustration
- Hint system available but not intrusive
- Multiple valid strategies per situation

## Web-Specific Considerations

### Browser Compatibility
- Target: Chrome, Firefox, Safari, Edge (latest 2 versions)
- WebGL 2.0 required
- SharedArrayBuffer support for threading (with fallback)

### Performance Targets
- First paint: < 2 seconds
- Interactive: < 5 seconds
- 60 FPS during gameplay
- Memory usage: < 100MB

### PWA Features
- Installable to home screen
- Offline play capability
- Auto-update when online
