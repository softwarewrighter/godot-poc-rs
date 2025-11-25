//! Match detection algorithms for the match-3 game

use crate::symbols::Grid;
use godot::prelude::*;

/// A match of 3 or more symbols
#[derive(Debug, Clone)]
pub struct Match {
    /// Positions of symbols in this match
    pub positions: Vec<Vector2i>,
    /// Whether this is a horizontal match
    pub horizontal: bool,
}

impl Match {
    /// Get the length of the match
    pub fn len(&self) -> usize {
        self.positions.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }

    /// Calculate score for this match
    pub fn score(&self) -> i32 {
        match self.len() {
            3 => 50,
            4 => 100,
            5 => 200,
            _ => 400,
        }
    }
}

/// Finds matches on the grid
pub struct MatchFinder;

impl MatchFinder {
    /// Find all horizontal matches
    pub fn find_horizontal(grid: &Grid) -> Vec<Match> {
        let mut matches = Vec::new();

        for y in 0..grid.height {
            let mut x = 0;
            while x < grid.width {
                if let Some(symbol) = grid.get(x, y) {
                    let symbol_type = symbol.current_type();
                    let mut match_len = 1;

                    // Count consecutive symbols of the same type
                    while x + match_len < grid.width {
                        if let Some(next) = grid.get(x + match_len, y) {
                            if next.current_type() == symbol_type {
                                match_len += 1;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    // If we found a match of 3+
                    if match_len >= 3 {
                        let positions: Vec<Vector2i> = (0..match_len)
                            .map(|i| Vector2i::new((x + i) as i32, y as i32))
                            .collect();

                        matches.push(Match {
                            positions,
                            horizontal: true,
                        });
                    }

                    x += match_len;
                } else {
                    x += 1;
                }
            }
        }

        matches
    }

    /// Find all vertical matches
    pub fn find_vertical(grid: &Grid) -> Vec<Match> {
        let mut matches = Vec::new();

        for x in 0..grid.width {
            let mut y = 0;
            while y < grid.height {
                if let Some(symbol) = grid.get(x, y) {
                    let symbol_type = symbol.current_type();
                    let mut match_len = 1;

                    // Count consecutive symbols of the same type
                    while y + match_len < grid.height {
                        if let Some(next) = grid.get(x, y + match_len) {
                            if next.current_type() == symbol_type {
                                match_len += 1;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    // If we found a match of 3+
                    if match_len >= 3 {
                        let positions: Vec<Vector2i> = (0..match_len)
                            .map(|i| Vector2i::new(x as i32, (y + i) as i32))
                            .collect();

                        matches.push(Match {
                            positions,
                            horizontal: false,
                        });
                    }

                    y += match_len;
                } else {
                    y += 1;
                }
            }
        }

        matches
    }

    /// Find all matches (horizontal and vertical)
    pub fn find_all(grid: &Grid) -> Vec<Match> {
        let mut matches = Self::find_horizontal(grid);
        matches.extend(Self::find_vertical(grid));
        matches
    }

    /// Get all unique positions that are part of any match
    pub fn get_matched_positions(matches: &[Match]) -> Vec<Vector2i> {
        let mut positions = Vec::new();
        for m in matches {
            for pos in &m.positions {
                if !positions.contains(pos) {
                    positions.push(*pos);
                }
            }
        }
        positions
    }

    /// Check if swapping two positions would create a match
    pub fn would_create_match(grid: &Grid, pos1: Vector2i, pos2: Vector2i) -> bool {
        // Create a temporary grid with swapped positions
        let mut temp_grid = grid.clone();

        let symbol1 = temp_grid.take(pos1.x as usize, pos1.y as usize);
        let symbol2 = temp_grid.take(pos2.x as usize, pos2.y as usize);

        if let (Some(mut s1), Some(mut s2)) = (symbol1, symbol2) {
            // Update grid positions
            s1.grid_pos = pos2;
            s2.grid_pos = pos1;

            temp_grid.set(pos2.x as usize, pos2.y as usize, Some(s1));
            temp_grid.set(pos1.x as usize, pos1.y as usize, Some(s2));

            // Check for matches
            let matches = Self::find_all(&temp_grid);
            !matches.is_empty()
        } else {
            false
        }
    }
}
