//! Game board implementation - the main Godot class

use crate::matching::MatchFinder;
use crate::symbols::{Grid, Symbol, SymbolType};
use godot::classes::{ColorRect, InputEvent, InputEventMouseButton, Node2D, Tween};
use godot::prelude::*;

/// Game states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameState {
    #[default]
    Ready,
    Selected,
    Swapping,
    Matching,
    Falling,
    Rotating,
}

/// The main game board - a Godot Node2D that manages the match-3 grid
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct GameBoard {
    base: Base<Node2D>,

    /// The logical grid
    #[var]
    grid_width: i32,
    #[var]
    grid_height: i32,

    grid: Grid,

    /// Visual representations of symbols (ColorRect nodes)
    symbol_nodes: Vec<Option<Gd<ColorRect>>>,

    /// Cell size in pixels
    #[var]
    cell_size: f32,

    /// Padding between cells
    #[var]
    cell_padding: f32,

    /// Currently selected position
    selected_pos: Option<Vector2i>,

    /// Current game state
    state: GameState,

    /// Score
    #[export]
    score: i32,

    /// Combo multiplier
    combo: i32,

    /// Rotation timer
    rotation_timer: f64,

    /// Rotation interval in seconds
    #[var]
    rotation_interval: f64,

    /// Board offset for centering
    board_offset: Vector2,
}

#[godot_api]
impl INode2D for GameBoard {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            grid_width: 8,
            grid_height: 8,
            grid: Grid::new(8, 8),
            symbol_nodes: Vec::new(),
            cell_size: 64.0,
            cell_padding: 4.0,
            selected_pos: None,
            state: GameState::Ready,
            score: 0,
            combo: 1,
            rotation_timer: 0.0,
            rotation_interval: 5.0,
            board_offset: Vector2::ZERO,
        }
    }

    fn ready(&mut self) {
        godot_print!("GameBoard ready - initializing {} x {} grid", self.grid_width, self.grid_height);
        self.initialize_board();
    }

    fn process(&mut self, delta: f64) {
        // Handle rotation timer
        if self.state == GameState::Ready {
            self.rotation_timer += delta;
            if self.rotation_timer >= self.rotation_interval {
                self.rotation_timer = 0.0;
                self.trigger_rotation();
            }
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.state != GameState::Ready && self.state != GameState::Selected {
            return;
        }

        if let Ok(mouse_event) = event.try_cast::<InputEventMouseButton>() {
            if mouse_event.is_pressed() && mouse_event.get_button_index() == godot::global::MouseButton::LEFT {
                let click_pos = mouse_event.get_position();
                self.handle_click(click_pos);
            }
        }
    }
}

#[godot_api]
impl GameBoard {
    /// Signal emitted when score changes
    #[signal]
    fn score_changed(new_score: i32);

    /// Signal emitted when a match is found
    #[signal]
    fn match_found(count: i32);

    /// Signal emitted when rotation occurs
    #[signal]
    fn rotation_triggered();

    /// Initialize the game board
    #[func]
    fn initialize_board(&mut self) {
        // Calculate board offset to center it
        let board_width = self.grid_width as f32 * self.cell_size;
        let board_height = self.grid_height as f32 * self.cell_size;
        self.board_offset = Vector2::new(
            (1280.0 - board_width) / 2.0,
            (720.0 - board_height) / 2.0,
        );

        // Create the grid
        self.grid = Grid::new(self.grid_width as usize, self.grid_height as usize);
        self.grid.fill_random();

        // Create visual nodes
        self.create_symbol_nodes();

        godot_print!("Board initialized with {} symbols", self.grid_width * self.grid_height);
    }

    /// Create visual nodes for all symbols
    fn create_symbol_nodes(&mut self) {
        // Clear existing nodes
        for node in self.symbol_nodes.drain(..) {
            if let Some(mut n) = node {
                n.queue_free();
            }
        }

        self.symbol_nodes = vec![None; (self.grid_width * self.grid_height) as usize];

        for y in 0..self.grid_height as usize {
            for x in 0..self.grid_width as usize {
                if let Some(symbol) = self.grid.get(x, y) {
                    let color = symbol.current_color();
                    let node = self.create_symbol_visual(x, y, color);
                    let idx = y * self.grid_width as usize + x;
                    self.symbol_nodes[idx] = Some(node);
                }
            }
        }
    }

    /// Create a visual representation of a symbol
    fn create_symbol_visual(&mut self, x: usize, y: usize, color: Color) -> Gd<ColorRect> {
        let mut rect = ColorRect::new_alloc();
        let size = self.cell_size - self.cell_padding * 2.0;

        rect.set_size(Vector2::new(size, size));
        rect.set_color(color);

        let pos = self.grid_to_screen(x as i32, y as i32);
        rect.set_position(pos);

        self.base_mut().add_child(&rect);
        rect
    }

    /// Convert grid coordinates to screen position
    fn grid_to_screen(&self, x: i32, y: i32) -> Vector2 {
        Vector2::new(
            self.board_offset.x + x as f32 * self.cell_size + self.cell_padding,
            self.board_offset.y + y as f32 * self.cell_size + self.cell_padding,
        )
    }

    /// Convert screen position to grid coordinates
    fn screen_to_grid(&self, pos: Vector2) -> Option<Vector2i> {
        let local_x = pos.x - self.board_offset.x;
        let local_y = pos.y - self.board_offset.y;

        if local_x < 0.0 || local_y < 0.0 {
            return None;
        }

        let grid_x = (local_x / self.cell_size) as i32;
        let grid_y = (local_y / self.cell_size) as i32;

        if grid_x >= 0 && grid_x < self.grid_width && grid_y >= 0 && grid_y < self.grid_height {
            Some(Vector2i::new(grid_x, grid_y))
        } else {
            None
        }
    }

    /// Handle a click on the board
    fn handle_click(&mut self, screen_pos: Vector2) {
        let Some(grid_pos) = self.screen_to_grid(screen_pos) else {
            return;
        };

        godot_print!("Clicked on grid position: {:?}", grid_pos);

        match self.state {
            GameState::Ready => {
                // Select this symbol
                self.select_symbol(grid_pos);
            }
            GameState::Selected => {
                if let Some(selected) = self.selected_pos {
                    if self.is_adjacent(selected, grid_pos) {
                        // Try to swap
                        self.try_swap(selected, grid_pos);
                    } else {
                        // Select new symbol instead
                        self.deselect_symbol();
                        self.select_symbol(grid_pos);
                    }
                }
            }
            _ => {}
        }
    }

    /// Check if two positions are adjacent
    fn is_adjacent(&self, pos1: Vector2i, pos2: Vector2i) -> bool {
        let dx = (pos1.x - pos2.x).abs();
        let dy = (pos1.y - pos2.y).abs();
        (dx == 1 && dy == 0) || (dx == 0 && dy == 1)
    }

    /// Select a symbol
    fn select_symbol(&mut self, pos: Vector2i) {
        self.selected_pos = Some(pos);
        self.state = GameState::Selected;

        // Visual feedback - scale up
        let idx = pos.y as usize * self.grid_width as usize + pos.x as usize;
        let size = self.cell_size - self.cell_padding * 2.0;
        let screen_pos = self.grid_to_screen(pos.x, pos.y);

        if let Some(Some(node)) = self.symbol_nodes.get_mut(idx) {
            node.set_size(Vector2::new(size + 8.0, size + 8.0));
            node.set_position(screen_pos - Vector2::new(4.0, 4.0));
        }

        godot_print!("Selected symbol at {:?}", pos);
    }

    /// Deselect current symbol
    fn deselect_symbol(&mut self) {
        if let Some(pos) = self.selected_pos.take() {
            // Reset visual
            let idx = pos.y as usize * self.grid_width as usize + pos.x as usize;
            let size = self.cell_size - self.cell_padding * 2.0;
            let screen_pos = self.grid_to_screen(pos.x, pos.y);

            if let Some(Some(node)) = self.symbol_nodes.get_mut(idx) {
                node.set_size(Vector2::new(size, size));
                node.set_position(screen_pos);
            }
        }
        self.state = GameState::Ready;
    }

    /// Try to swap two symbols
    fn try_swap(&mut self, pos1: Vector2i, pos2: Vector2i) {
        // Check if swap would create a match
        if !MatchFinder::would_create_match(&self.grid, pos1, pos2) {
            godot_print!("Invalid swap - no match would be created");
            self.deselect_symbol();
            return;
        }

        godot_print!("Swapping {:?} with {:?}", pos1, pos2);
        self.state = GameState::Swapping;

        // Perform the swap
        self.swap_symbols(pos1, pos2);

        // Process matches
        self.process_matches();
    }

    /// Swap two symbols in the grid and visually
    fn swap_symbols(&mut self, pos1: Vector2i, pos2: Vector2i) {
        // Swap in grid
        let symbol1 = self.grid.take(pos1.x as usize, pos1.y as usize);
        let symbol2 = self.grid.take(pos2.x as usize, pos2.y as usize);

        if let (Some(mut s1), Some(mut s2)) = (symbol1, symbol2) {
            s1.grid_pos = pos2;
            s2.grid_pos = pos1;

            self.grid.set(pos2.x as usize, pos2.y as usize, Some(s1));
            self.grid.set(pos1.x as usize, pos1.y as usize, Some(s2));
        }

        // Swap visual nodes
        let idx1 = pos1.y as usize * self.grid_width as usize + pos1.x as usize;
        let idx2 = pos2.y as usize * self.grid_width as usize + pos2.x as usize;

        self.symbol_nodes.swap(idx1, idx2);

        // Calculate positions before borrowing
        let size = self.cell_size - self.cell_padding * 2.0;
        let screen_pos1 = self.grid_to_screen(pos1.x, pos1.y);
        let screen_pos2 = self.grid_to_screen(pos2.x, pos2.y);

        // Update positions
        if let Some(Some(node)) = self.symbol_nodes.get_mut(idx1) {
            node.set_size(Vector2::new(size, size));
            node.set_position(screen_pos1);
        }
        if let Some(Some(node)) = self.symbol_nodes.get_mut(idx2) {
            node.set_size(Vector2::new(size, size));
            node.set_position(screen_pos2);
        }
    }

    /// Process all matches on the board
    fn process_matches(&mut self) {
        let matches = MatchFinder::find_all(&self.grid);

        if matches.is_empty() {
            self.combo = 1;
            self.state = GameState::Ready;
            self.selected_pos = None;
            return;
        }

        self.state = GameState::Matching;

        // Calculate score
        let mut match_score = 0;
        for m in &matches {
            match_score += m.score() * self.combo;
        }
        self.score += match_score;

        // Emit signals
        let match_count = matches.len() as i32;
        let current_score = self.score;
        self.base_mut().emit_signal("match_found", &[match_count.to_variant()]);
        self.base_mut().emit_signal("score_changed", &[current_score.to_variant()]);

        godot_print!("Found {} matches, score: {} (combo: {}x)", matches.len(), self.score, self.combo);

        // Increment combo for cascades
        self.combo += 1;

        // Clear matched symbols with animation
        let positions = MatchFinder::get_matched_positions(&matches);
        self.animate_clear_symbols(&positions);
    }

    /// Animate clearing symbols, then trigger gravity
    fn animate_clear_symbols(&mut self, positions: &[Vector2i]) {
        let clear_duration = 0.2;

        // Collect nodes to animate
        let mut nodes_to_clear: Vec<Gd<ColorRect>> = Vec::new();
        for pos in positions {
            let idx = pos.y as usize * self.grid_width as usize + pos.x as usize;
            if let Some(Some(node)) = self.symbol_nodes.get(idx) {
                nodes_to_clear.push(node.clone());
            }
            // Remove from grid immediately (logical state)
            self.grid.set(pos.x as usize, pos.y as usize, None);
        }

        // Store positions for later cleanup
        let positions_vec: Vec<Vector2i> = positions.to_vec();

        // Get callable for after animation
        let callable = self.base().callable("on_clear_complete");

        // Create shrink/fade animation
        if let Some(mut tween) = self.base_mut().create_tween() {
            tween.set_parallel();

            let zero_scale = Variant::from(Vector2::ZERO);
            for node in &nodes_to_clear {
                // Shrink to nothing
                tween.tween_property(
                    node,
                    "scale",
                    &zero_scale,
                    clear_duration,
                );
            }

            // After animation, clean up and apply gravity
            tween.chain();
            tween.tween_callback(&callable);
        }

        // Store positions to clear in a temporary way - we'll clean up in callback
        // For now, mark which indices need cleanup
        for pos in &positions_vec {
            let idx = pos.y as usize * self.grid_width as usize + pos.x as usize;
            if let Some(node_opt) = self.symbol_nodes.get_mut(idx) {
                // We'll free these in the callback
                *node_opt = None;
            }
        }

        // Queue free the nodes after animation
        for mut node in nodes_to_clear {
            // Use a separate tween to delay the queue_free
            if let Some(mut delay_tween) = self.base_mut().create_tween() {
                let free_callable = node.callable("queue_free");
                delay_tween.tween_interval(clear_duration + 0.01);
                delay_tween.tween_callback(&free_callable);
            }
        }
    }

    /// Called when clear animation completes
    #[func]
    fn on_clear_complete(&mut self) {
        godot_print!("Clear animation complete, applying gravity");
        self.animate_gravity();
    }

    /// Clear symbols at the given positions
    fn clear_symbols(&mut self, positions: &[Vector2i]) {
        for pos in positions {
            let idx = pos.y as usize * self.grid_width as usize + pos.x as usize;

            // Remove from grid
            self.grid.set(pos.x as usize, pos.y as usize, None);

            // Remove visual
            if let Some(Some(mut node)) = self.symbol_nodes.get_mut(idx).map(|n| n.take()) {
                node.queue_free();
            }
        }
    }

    /// Apply gravity - make symbols fall down (no animation, instant)
    fn apply_gravity(&mut self) {
        self.state = GameState::Falling;

        for x in 0..self.grid_width as usize {
            let mut write_y = self.grid_height as usize - 1;

            for read_y in (0..self.grid_height as usize).rev() {
                if self.grid.get(x, read_y).is_some() {
                    if read_y != write_y {
                        // Move symbol down
                        let symbol = self.grid.take(x, read_y);
                        if let Some(mut s) = symbol {
                            s.grid_pos = Vector2i::new(x as i32, write_y as i32);
                            self.grid.set(x, write_y, Some(s));
                        }

                        // Move visual node
                        let from_idx = read_y * self.grid_width as usize + x;
                        let to_idx = write_y * self.grid_width as usize + x;

                        let screen_pos = self.grid_to_screen(x as i32, write_y as i32);
                        let node = self.symbol_nodes[from_idx].take();
                        if let Some(mut n) = node {
                            n.set_position(screen_pos);
                            self.symbol_nodes[to_idx] = Some(n);
                        }
                    }
                    write_y = write_y.saturating_sub(1);
                }
            }
        }
    }

    /// Apply gravity with falling animation
    fn animate_gravity(&mut self) {
        self.state = GameState::Falling;

        let fall_duration_per_cell = 0.08;

        // Collect all moves needed: (from_idx, to_idx, from_y, to_y, x)
        let mut moves: Vec<(usize, usize, usize, usize, usize)> = Vec::new();

        for x in 0..self.grid_width as usize {
            let mut write_y = self.grid_height as usize - 1;

            for read_y in (0..self.grid_height as usize).rev() {
                if self.grid.get(x, read_y).is_some() {
                    if read_y != write_y {
                        let from_idx = read_y * self.grid_width as usize + x;
                        let to_idx = write_y * self.grid_width as usize + x;
                        moves.push((from_idx, to_idx, read_y, write_y, x));

                        // Update logical grid
                        let symbol = self.grid.take(x, read_y);
                        if let Some(mut s) = symbol {
                            s.grid_pos = Vector2i::new(x as i32, write_y as i32);
                            self.grid.set(x, write_y, Some(s));
                        }
                    }
                    write_y = write_y.saturating_sub(1);
                }
            }
        }

        if moves.is_empty() {
            // No falling needed, go straight to refill
            self.animate_refill();
            return;
        }

        // Collect nodes and target positions
        let mut animations: Vec<(Gd<ColorRect>, Vector2, f64)> = Vec::new();
        for (from_idx, to_idx, from_y, to_y, x) in &moves {
            if let Some(node) = self.symbol_nodes[*from_idx].take() {
                let target_pos = self.grid_to_screen(*x as i32, *to_y as i32);
                let distance = *to_y - *from_y;
                let duration = distance as f64 * fall_duration_per_cell;
                animations.push((node, target_pos, duration));
                // Will be placed at to_idx after animation
            }
        }

        // Calculate max duration for callback timing
        let max_duration = animations.iter().map(|(_, _, d)| *d).fold(0.0, f64::max);

        // Get callback
        let callable = self.base().callable("on_gravity_complete");

        // Create fall animations
        if let Some(mut tween) = self.base_mut().create_tween() {
            tween.set_parallel();

            for (node, target_pos, duration) in &animations {
                let pos_variant = Variant::from(*target_pos);
                tween.tween_property(
                    node,
                    "position",
                    &pos_variant,
                    *duration,
                );
            }

            tween.chain();
            tween.tween_callback(&callable);
        }

        // Update symbol_nodes array with new positions
        for (i, (from_idx, to_idx, _, _, _)) in moves.iter().enumerate() {
            if i < animations.len() {
                self.symbol_nodes[*to_idx] = Some(animations[i].0.clone());
            }
        }
    }

    /// Called when gravity animation completes
    #[func]
    fn on_gravity_complete(&mut self) {
        godot_print!("Gravity complete, refilling board");
        self.animate_refill();
    }

    /// Refill empty spaces with animation
    fn animate_refill(&mut self) {
        let spawn_duration = 0.15;

        // Find empty cells and create new symbols
        let mut new_symbols: Vec<(usize, usize, Gd<ColorRect>)> = Vec::new();

        for x in 0..self.grid_width as usize {
            for y in 0..self.grid_height as usize {
                if self.grid.get(x, y).is_none() {
                    // Create new symbol
                    let symbol_type = SymbolType::random();
                    let symbol = Symbol::with_type(Vector2i::new(x as i32, y as i32), symbol_type);
                    let color = symbol_type.color();

                    self.grid.set(x, y, Some(symbol));

                    // Create visual starting from above the board
                    let mut rect = ColorRect::new_alloc();
                    let size = self.cell_size - self.cell_padding * 2.0;
                    rect.set_size(Vector2::new(size, size));
                    rect.set_color(color);

                    // Start above the board
                    let start_pos = Vector2::new(
                        self.board_offset.x + x as f32 * self.cell_size + self.cell_padding,
                        -self.cell_size, // Above the visible area
                    );
                    rect.set_position(start_pos);
                    rect.set_scale(Vector2::new(1.0, 1.0)); // Ensure scale is reset

                    self.base_mut().add_child(&rect);

                    new_symbols.push((x, y, rect));
                }
            }
        }

        if new_symbols.is_empty() {
            // No refill needed, check for cascades
            self.on_refill_complete();
            return;
        }

        // Pre-calculate target positions
        let target_positions: Vec<(Vector2, f64)> = new_symbols
            .iter()
            .map(|(x, y, _)| {
                let target_pos = self.grid_to_screen(*x as i32, *y as i32);
                let duration = spawn_duration + (*y as f64 * 0.05);
                (target_pos, duration)
            })
            .collect();

        // Get callback
        let callable = self.base().callable("on_refill_complete");

        // Animate new symbols falling into place
        if let Some(mut tween) = self.base_mut().create_tween() {
            tween.set_parallel();

            for (i, (_, _, node)) in new_symbols.iter().enumerate() {
                let (target_pos, duration) = target_positions[i];
                let pos_variant = Variant::from(target_pos);
                tween.tween_property(
                    node,
                    "position",
                    &pos_variant,
                    duration,
                );
            }

            tween.chain();
            tween.tween_callback(&callable);
        }

        // Store nodes in symbol_nodes
        for (x, y, node) in new_symbols {
            let idx = y * self.grid_width as usize + x;
            self.symbol_nodes[idx] = Some(node);
        }
    }

    /// Called when refill animation completes
    #[func]
    fn on_refill_complete(&mut self) {
        godot_print!("Refill complete, checking for cascades");
        // Check for new matches (cascades)
        self.process_matches();
    }

    /// Refill empty spaces with new symbols
    fn refill_board(&mut self) {
        for x in 0..self.grid_width as usize {
            for y in 0..self.grid_height as usize {
                if self.grid.get(x, y).is_none() {
                    // Create new symbol
                    let symbol_type = SymbolType::random();
                    let symbol = Symbol::with_type(Vector2i::new(x as i32, y as i32), symbol_type);
                    let color = symbol_type.color();

                    self.grid.set(x, y, Some(symbol));

                    // Create visual
                    let node = self.create_symbol_visual(x, y, color);
                    let idx = y * self.grid_width as usize + x;
                    self.symbol_nodes[idx] = Some(node);
                }
            }
        }
    }

    /// Trigger rotation of all symbols
    fn trigger_rotation(&mut self) {
        if self.state != GameState::Ready {
            return;
        }

        godot_print!("Triggering rotation!");
        self.state = GameState::Rotating;

        let rotation_duration = 0.3;
        let size = self.cell_size - self.cell_padding * 2.0;

        // Set pivot to center for all symbols first
        for idx in 0..self.symbol_nodes.len() {
            if let Some(Some(node)) = self.symbol_nodes.get_mut(idx) {
                node.set_pivot_offset(Vector2::new(size / 2.0, size / 2.0));
            }
        }

        // Collect nodes to animate (to avoid borrow issues)
        let nodes_to_animate: Vec<Gd<ColorRect>> = self.symbol_nodes
            .iter()
            .filter_map(|opt| opt.as_ref().map(|n| n.clone()))
            .collect();

        // Get callable before creating tween
        let callable = self.base().callable("finish_rotation");

        // Create tween for rotation animation
        if let Some(mut tween) = self.base_mut().create_tween() {
            tween.set_parallel(); // Enable parallel tweening

            // Animate each symbol's rotation by 90 degrees
            let final_rotation = Variant::from(std::f64::consts::FRAC_PI_2); // 90 degrees
            for node in &nodes_to_animate {
                tween.tween_property(
                    node,
                    "rotation",
                    &final_rotation,
                    rotation_duration,
                );
            }

            // Chain a callback to finish the rotation (after parallel tweens complete)
            tween.chain(); // Switch back to sequential
            tween.tween_callback(&callable);
        }

        self.base_mut().emit_signal("rotation_triggered", &[]);
    }

    /// Called when rotation animation finishes
    #[func]
    fn finish_rotation(&mut self) {
        godot_print!("Finishing rotation");

        // Rotate the logical grid
        self.grid.rotate_all();

        // Update visual colors and reset rotation angle
        for y in 0..self.grid_height as usize {
            for x in 0..self.grid_width as usize {
                if let Some(symbol) = self.grid.get(x, y) {
                    let color = symbol.current_color();
                    let idx = y * self.grid_width as usize + x;
                    if let Some(Some(node)) = self.symbol_nodes.get_mut(idx) {
                        node.set_color(color);
                        node.set_rotation(0.0);
                    }
                }
            }
        }

        // Check for new matches after rotation
        self.state = GameState::Ready;
        self.process_matches();
    }

    /// Reset the board
    #[func]
    fn reset(&mut self) {
        self.score = 0;
        self.combo = 1;
        self.rotation_timer = 0.0;
        self.state = GameState::Ready;
        self.selected_pos = None;
        self.initialize_board();
    }
}
