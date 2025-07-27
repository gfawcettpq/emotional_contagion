// 🌀 QUANTUM EMOTION TUNNELING - Game 1
// Emotions can spontaneously tunnel across space quantum-mechanically!

use macroquad::prelude::*;

mod simulation;
use simulation::{emotions::*, grid::*, entity::*};

/// Game state for quantum emotion tunneling system
struct GameState {
    grid: EmotionGrid,
    toolbox: CharacterToolbox,
    paused: bool,
    show_help: bool,
    quantum_events: u32,
}

impl GameState {
    fn new() -> Self {
        println!("🌀 QUANTUM EMOTION TUNNELING - Conway's Game of Life");
        println!("🎮 Screen size: {}x{}", screen_width(), screen_height());
        
        // Create ultra-high resolution grid (2px cells for quantum detail)
        let cell_size = 2.0;
        let grid_width = (screen_width() / cell_size) as usize;
        let grid_height = (screen_height() / cell_size) as usize;
        let grid = EmotionGrid::new(grid_width, grid_height, cell_size);
        
        println!("🗂️ Created quantum grid: {}x{} cells ({}px each)", grid_width, grid_height, cell_size);
        
        // Create compact toolbox
        let toolbox_width = 280.0;
        let toolbox_height = 300.0;
        let toolbox_x = screen_width() - toolbox_width - 10.0;
        let toolbox_y = 10.0;
        let toolbox = CharacterToolbox::new(toolbox_x, toolbox_y, toolbox_width, toolbox_height);
        
        Self {
            grid,
            toolbox,
            paused: false,
            show_help: false,
            quantum_events: 0,
        }
    }

    fn update(&mut self, _dt: f32) {
        if !self.paused {
            self.grid.update();
            
            // QUANTUM TUNNELING EVENTS - emotions randomly teleport!
            use ::rand::{Rng, thread_rng};
            let mut rng = thread_rng();
            
            // 5% chance per frame for quantum tunneling event
            if rng.gen_bool(0.05) {
                self.perform_quantum_tunneling();
            }
        }
    }
    
    fn perform_quantum_tunneling(&mut self) {
        use ::rand::{Rng, thread_rng};
        let mut rng = thread_rng();
        
        // Find a random cell with strong emotion
        for _ in 0..20 { // Try 20 times to find a good source
            let source_x = rng.gen_range(0..self.grid.width);
            let source_y = rng.gen_range(0..self.grid.height);
            
            let source_cell = self.grid.grid[source_y][source_x].clone(); // Clone to avoid borrow
            if source_cell.alive && source_cell.emotion.is_some() && source_cell.intensity > 0.7 {
                // Quantum tunnel to random location
                let target_x = rng.gen_range(0..self.grid.width);
                let target_y = rng.gen_range(0..self.grid.height);
                
                // Make target cell alive with tunneled emotion
                self.grid.grid[target_y][target_x] = EmotionCell::new_with_emotion(
                    source_cell.emotion.unwrap(),
                    source_cell.intensity * 0.8
                );
                
                self.quantum_events += 1;
                if self.quantum_events % 10 == 0 {
                    println!("⚡ QUANTUM TUNNEL #{}: {} teleported", 
                        self.quantum_events, source_cell.emotion.unwrap().name());
                }
                break;
            }
        }
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.paused = !self.paused;
            println!("⏯️ Game {}", if self.paused { "PAUSED" } else { "RESUMED" });
        }

        if is_key_pressed(KeyCode::R) {
            self.grid.randomize();
            println!("🎲 Quantum grid randomized");
        }

        if is_key_pressed(KeyCode::C) {
            self.grid.clear();
            self.quantum_events = 0;
            println!("🗑️ Quantum grid cleared");
        }

        if is_key_pressed(KeyCode::H) {
            self.show_help = !self.show_help;
        }

        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }

        // Mouse input for drag and drop
        let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);

        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some(character) = self.toolbox.get_character_at_pos(mouse_pos) {
                self.toolbox.start_drag(character.clone(), mouse_pos);
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if let Some(character) = self.toolbox.stop_drag() {
                self.grid.add_emotion_source(character.emotion, mouse_pos.x, mouse_pos.y);
            }
        }
    }

    fn render(&mut self) {
        // Quantum void background (deeper than normal space)
        let background = Color::new(0.05, 0.05, 0.15, 1.0); // Deep quantum blue
        clear_background(background);
        
        // Render the quantum emotion grid
        self.grid.render();
        
        // Render quantum stats
        self.render_quantum_stats();
        
        // Render toolbox
        self.toolbox.render();
        
        // Render dragging character if active
        let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
        self.toolbox.render_dragging(mouse_pos);
        
        if self.show_help {
            self.render_help_overlay();
        }
    }

    fn render_quantum_stats(&self) {
        let panel_bg = Color::new(0.05, 0.05, 0.25, 0.9); // Quantum blue overlay
        let text_primary = Color::new(0.80, 0.90, 1.0, 1.0); // Quantum white
        let accent = Color::new(0.40, 0.80, 1.0, 1.0); // Quantum cyan

        let stats = self.grid.get_stats();
        
        let x = 10.0;
        let y = 10.0;
        let width = 380.0;
        let height = 90.0;

        draw_rectangle(x, y, width, height, panel_bg);
        
        // Status line with quantum info
        let status_text = format!("🌀 QUANTUM: {} tunnels | 🌊 Alive: {} | 💫 Emotions: {} | #{}", 
            self.quantum_events, stats.alive_count, stats.emotion_count, self.grid.update_count);
        draw_text(&status_text, x + 5.0, y + 20.0, 14.0, text_primary);
        
        draw_text("⚡ Emotions randomly tunnel across space quantum-mechanically!", x + 5.0, y + 40.0, 12.0, accent);
        draw_text("SPACE:Pause R:Random C:Clear H:Help ESC:Exit", x + 5.0, y + 60.0, 12.0, accent);
        draw_text("🎭 Drag emotions from toolbox to add quantum sources →", x + 5.0, y + 80.0, 12.0, accent);
    }

    fn render_help_overlay(&self) {
        let overlay_color = Color::new(0.05, 0.05, 0.25, 0.8);
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), overlay_color);

        let panel_bg = Color::new(0.10, 0.10, 0.35, 1.0);
        let accent = Color::new(0.40, 0.80, 1.0, 1.0);
        let text_primary = Color::new(0.90, 0.95, 1.0, 1.0);

        let help_width = 600.0;
        let help_height = 400.0;
        let help_x = (screen_width() - help_width) / 2.0;
        let help_y = (screen_height() - help_height) / 2.0;

        draw_rectangle(help_x, help_y, help_width, help_height, panel_bg);
        draw_rectangle_lines(help_x, help_y, help_width, help_height, 3.0, accent);

        draw_text("🌀 QUANTUM EMOTION TUNNELING", help_x + 20.0, help_y + 40.0, 24.0, accent);

        let help_content = [
            "⚡ Emotions defy physics and tunnel across space!",
            "",
            "• Normal Conway's Game of Life spreading",
            "• PLUS quantum tunneling events",
            "• Watch emotions appear far from sources",
            "• Spooky action at a distance!",
            "",
            "Drag emotions from toolbox to create sources",
            "SPACE:Pause R:Random C:Clear H:Help ESC:Exit",
        ];

        let mut content_y = help_y + 80.0;
        for line in &help_content {
            if content_y + 20.0 < help_y + help_height {
                draw_text(line, help_x + 20.0, content_y, 16.0, text_primary);
                content_y += 22.0;
            }
        }
    }
}

#[macroquad::main("🌀 Quantum Emotion Tunneling")]
async fn main() {
    println!("🌀 QUANTUM EMOTION TUNNELING - Game 1 of Dragon's Collection");
    println!("⚡ Emotions defy classical physics and tunnel across space!");
    
    let mut game_state = GameState::new();
    
    // Add some initial quantum sources
    game_state.grid.add_emotion_source(EmotionType::Joy, 100.0, 100.0);
    game_state.grid.add_emotion_source(EmotionType::Fear, 300.0, 200.0);
    game_state.grid.add_emotion_source(EmotionType::Love, 500.0, 300.0);
    
    println!("🚀 Quantum field initialized - tunneling events will begin!");
    
    loop {
        let dt = get_frame_time();
        
        game_state.handle_input();
        game_state.update(dt);
        game_state.render();
        
        next_frame().await;
    }
}