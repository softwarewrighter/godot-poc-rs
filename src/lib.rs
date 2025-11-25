//! Revolving Match-3 - A match-3 puzzle game built with Rust and Godot
//!
//! This crate provides the core game logic for a match-3 game with a unique
//! revolving symbols mechanic, implemented in Rust via gdext.

use godot::prelude::*;

mod board;
mod matching;
mod symbols;

struct RevolvingMatch3Extension;

#[gdextension]
unsafe impl ExtensionLibrary for RevolvingMatch3Extension {}
