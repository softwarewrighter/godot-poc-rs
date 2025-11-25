//! Symbol types and management for the match-3 game

use godot::prelude::*;
use rand::Rng;

/// The different symbol types available in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SymbolType {
    #[default]
    Red,
    Blue,
    Green,
    Yellow,
    Purple,
    Orange,
}

impl SymbolType {
    /// All available symbol types
    pub const ALL: [SymbolType; 6] = [
        SymbolType::Red,
        SymbolType::Blue,
        SymbolType::Green,
        SymbolType::Yellow,
        SymbolType::Purple,
        SymbolType::Orange,
    ];

    /// Get a random symbol type
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self::ALL[rng.gen_range(0..Self::ALL.len())]
    }

    /// Get the color for this symbol type
    pub fn color(&self) -> Color {
        match self {
            SymbolType::Red => Color::from_rgb(0.9, 0.2, 0.2),
            SymbolType::Blue => Color::from_rgb(0.2, 0.4, 0.9),
            SymbolType::Green => Color::from_rgb(0.2, 0.8, 0.2),
            SymbolType::Yellow => Color::from_rgb(0.95, 0.85, 0.2),
            SymbolType::Purple => Color::from_rgb(0.6, 0.2, 0.8),
            SymbolType::Orange => Color::from_rgb(0.95, 0.5, 0.1),
        }
    }

    /// Get the index of this symbol type
    pub fn index(&self) -> usize {
        match self {
            SymbolType::Red => 0,
            SymbolType::Blue => 1,
            SymbolType::Green => 2,
            SymbolType::Yellow => 3,
            SymbolType::Purple => 4,
            SymbolType::Orange => 5,
        }
    }

    /// Get symbol type from index
    pub fn from_index(index: usize) -> Self {
        Self::ALL[index % Self::ALL.len()]
    }
}

/// A symbol on the game board with rotation capability
#[derive(Debug, Clone)]
pub struct Symbol {
    /// The four faces of this symbol (for rotation)
    pub faces: [SymbolType; 4],
    /// Current rotation state (0-3)
    pub rotation_state: u8,
    /// Position on the grid
    pub grid_pos: Vector2i,
    /// Whether this symbol is selected
    pub selected: bool,
    /// Whether this symbol is marked for clearing
    pub marked_for_clear: bool,
}

impl Symbol {
    /// Create a new symbol with random faces
    pub fn new(grid_pos: Vector2i) -> Self {
        let mut rng = rand::thread_rng();
        let faces = [
            SymbolType::from_index(rng.gen_range(0..6)),
            SymbolType::from_index(rng.gen_range(0..6)),
            SymbolType::from_index(rng.gen_range(0..6)),
            SymbolType::from_index(rng.gen_range(0..6)),
        ];

        Self {
            faces,
            rotation_state: 0,
            grid_pos,
            selected: false,
            marked_for_clear: false,
        }
    }

    /// Create a symbol with a specific current type
    /// Each symbol type has a fixed rotation cycle so patterns stay consistent
    pub fn with_type(grid_pos: Vector2i, symbol_type: SymbolType) -> Self {
        // Each symbol type rotates through a fixed sequence of 4 colors
        // This keeps the board pattern consistent when all symbols rotate together
        let faces = match symbol_type {
            SymbolType::Red => [SymbolType::Red, SymbolType::Blue, SymbolType::Green, SymbolType::Yellow],
            SymbolType::Blue => [SymbolType::Blue, SymbolType::Green, SymbolType::Yellow, SymbolType::Purple],
            SymbolType::Green => [SymbolType::Green, SymbolType::Yellow, SymbolType::Purple, SymbolType::Orange],
            SymbolType::Yellow => [SymbolType::Yellow, SymbolType::Purple, SymbolType::Orange, SymbolType::Red],
            SymbolType::Purple => [SymbolType::Purple, SymbolType::Orange, SymbolType::Red, SymbolType::Blue],
            SymbolType::Orange => [SymbolType::Orange, SymbolType::Red, SymbolType::Blue, SymbolType::Green],
        };

        Self {
            faces,
            rotation_state: 0,
            grid_pos,
            selected: false,
            marked_for_clear: false,
        }
    }

    /// Get the current symbol type (based on rotation)
    pub fn current_type(&self) -> SymbolType {
        self.faces[self.rotation_state as usize]
    }

    /// Get the current color
    pub fn current_color(&self) -> Color {
        self.current_type().color()
    }

    /// Rotate the symbol clockwise
    pub fn rotate(&mut self) {
        self.rotation_state = (self.rotation_state + 1) % 4;
    }

    /// Check if this symbol matches another (same current type)
    pub fn matches(&self, other: &Symbol) -> bool {
        self.current_type() == other.current_type()
    }
}

/// Grid of symbols
#[derive(Debug, Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Option<Symbol>>,
}

impl Grid {
    /// Create a new empty grid
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![None; width * height],
        }
    }

    /// Get the index for a position
    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    /// Check if a position is valid
    pub fn is_valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }

    /// Get a cell
    pub fn get(&self, x: usize, y: usize) -> Option<&Symbol> {
        if x < self.width && y < self.height {
            self.cells[self.index(x, y)].as_ref()
        } else {
            None
        }
    }

    /// Get a mutable cell
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Symbol> {
        if x < self.width && y < self.height {
            let idx = self.index(x, y);
            self.cells[idx].as_mut()
        } else {
            None
        }
    }

    /// Set a cell
    pub fn set(&mut self, x: usize, y: usize, symbol: Option<Symbol>) {
        if x < self.width && y < self.height {
            let idx = self.index(x, y);
            self.cells[idx] = symbol;
        }
    }

    /// Take a symbol from a cell (removes it)
    pub fn take(&mut self, x: usize, y: usize) -> Option<Symbol> {
        if x < self.width && y < self.height {
            let idx = self.index(x, y);
            self.cells[idx].take()
        } else {
            None
        }
    }

    /// Fill the grid with random symbols (avoiding initial matches)
    pub fn fill_random(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut symbol_type = SymbolType::random();

                // Avoid creating matches on fill
                let mut attempts = 0;
                while attempts < 10 {
                    let would_match_h = x >= 2
                        && self.get(x - 1, y).is_some_and(|s| s.current_type() == symbol_type)
                        && self.get(x - 2, y).is_some_and(|s| s.current_type() == symbol_type);

                    let would_match_v = y >= 2
                        && self.get(x, y - 1).is_some_and(|s| s.current_type() == symbol_type)
                        && self.get(x, y - 2).is_some_and(|s| s.current_type() == symbol_type);

                    if !would_match_h && !would_match_v {
                        break;
                    }
                    symbol_type = SymbolType::random();
                    attempts += 1;
                }

                let symbol = Symbol::with_type(Vector2i::new(x as i32, y as i32), symbol_type);
                self.set(x, y, Some(symbol));
            }
        }
    }

    /// Rotate all symbols
    pub fn rotate_all(&mut self) {
        for cell in &mut self.cells {
            if let Some(symbol) = cell {
                symbol.rotate();
            }
        }
    }
}
