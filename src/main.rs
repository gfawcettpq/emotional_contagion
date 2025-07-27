// Simple Emotion Contagion - Conway's Game of Life with emotions
// Based on the Python version - drag and drop emotion sources, watch them spread!

use macroquad::prelude::*;

mod simulation;
use simulation::{emotions::*, grid::*, entity::*};

/// Game state for our simple emotion contagion system
struct GameState {
    grid: EmotionGrid,
    toolbox: CharacterToolbox,
    paused: bool,
    show_help: bool,
}

impl GameState {
    fn new() -> Self {
        println!("🌊 Emotion Contagion - Conway's Game of Life");
        println!("🎮 Screen size: {}x{}", screen_width(), screen_height());
        
        // Create grid that fills most of the screen (6px cells like Python)
        let cell_size = 6.0;
        let grid_width = (screen_width() / cell_size) as usize;
        let grid_height = (screen_height() / cell_size) as usize;
        let grid = EmotionGrid::new(grid_width, grid_height, cell_size);
        
        println!("🗂️ Created grid: {}x{} cells ({}px each)", grid_width, grid_height, cell_size);
        
        // Create toolbox on the right side
        let toolbox_width = 350.0;
        let toolbox_height = 400.0;
        let toolbox_x = screen_width() - toolbox_width - 20.0;
        let toolbox_y = 20.0;
        let toolbox = CharacterToolbox::new(toolbox_x, toolbox_y, toolbox_width, toolbox_height);
        
        Self {
            grid,
            toolbox,
            paused: false,
            show_help: false,
        }
    }

    fn update(&mut self, _dt: f32) {
        if !self.paused {
            self.grid.update();
        }
    }

    fn handle_input(&mut self) {
        // Keyboard input
        if is_key_pressed(KeyCode::Space) {
            self.paused = !self.paused;
            println!("⏯️ Game {}", if self.paused { "PAUSED" } else { "RESUMED" });
        }

        if is_key_pressed(KeyCode::R) {
            self.grid.randomize();
            println!("🎲 Grid randomized");
        }

        if is_key_pressed(KeyCode::C) {
            self.grid.clear();
            println!("🗑️ Grid cleared");
        }

        if is_key_pressed(KeyCode::H) {
            self.show_help = !self.show_help;
            println!("❓ Help {}", if self.show_help { "ON" } else { "OFF" });
        }

        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }

        // Mouse input for drag and drop
        let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);

        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some(character) = self.toolbox.get_character_at_pos(mouse_pos) {
                self.toolbox.start_drag(character.clone(), mouse_pos);
                println!("🎭 Started dragging {}", character.name);
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if let Some(character) = self.toolbox.stop_drag() {
                // Add character to grid at mouse position
                self.grid.add_emotion_source(character.emotion, mouse_pos.x, mouse_pos.y);
                println!("🎭 Dropped {} at ({:.0}, {:.0})", character.name, mouse_pos.x, mouse_pos.y);
            }
        }
    }

    fn render(&mut self) {
        // Nord background color
        let background = Color::new(0.18, 0.20, 0.25, 1.0); // #2e3440
        clear_background(background);
        
        // Render the emotion grid
        self.grid.render();
        
        // Render UI panels
        self.render_info_panel();
        self.render_stats_panel();
        self.render_rules_panel();
        
        // Render toolbox
        self.toolbox.render();
        
        // Render dragging character if active
        let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
        self.toolbox.render_dragging(mouse_pos);
        
        // Render help overlay if active
        if self.show_help {
            self.render_help_overlay();
        }
    }

    fn render_info_panel(&self) {
        let panel_bg = Color::new(0.23, 0.26, 0.32, 1.0);  // #3b4252
        let panel_border = Color::new(0.26, 0.30, 0.37, 1.0); // #434c5e
        let accent = Color::new(0.53, 0.75, 0.82, 1.0); // #88c0d0
        let text_primary = Color::new(0.90, 0.91, 0.94, 1.0); // #e5e9f0

        let x = 20.0;
        let y = 20.0;
        let width = 300.0;
        let height = 200.0;

        // Panel background
        draw_rectangle(x, y, width, height, panel_bg);
        draw_rectangle_lines(x, y, width, height, 2.0, panel_border);

        // Title
        draw_text("ℹ️ Information", x + 10.0, y + 30.0, 20.0, accent);

        // Content
        let content = [
            "🌊 Conway's Game of Life",
            "with Emotion Contagion",
            "",
            "Click and drag characters",
            "from the toolbox to",
            "add them to the grid.",
            "",
            "Watch emotions spread",
            "through the cellular",
            "automata patterns."
        ];

        let mut content_y = y + 50.0;
        for line in &content {
            draw_text(line, x + 10.0, content_y, 16.0, text_primary);
            content_y += 18.0;
        }
    }

    fn render_stats_panel(&self) {
        let panel_bg = Color::new(0.23, 0.26, 0.32, 1.0);  // #3b4252
        let panel_border = Color::new(0.26, 0.30, 0.37, 1.0); // #434c5e
        let accent = Color::new(0.53, 0.75, 0.82, 1.0); // #88c0d0
        let text_primary = Color::new(0.90, 0.91, 0.94, 1.0); // #e5e9f0

        let stats = self.grid.get_stats();
        let width = 300.0;
        let height = 250.0;
        let x = screen_width() - width - 20.0;
        let y = screen_height() - height - 20.0;

        // Panel background
        draw_rectangle(x, y, width, height, panel_bg);
        draw_rectangle_lines(x, y, width, height, 2.0, panel_border);

        // Title
        draw_text("📊 Statistics", x + 10.0, y + 30.0, 20.0, accent);

        // Content
        let content = [
            format!("Alive Cells: {}", stats.alive_count),
            format!("Emotion Sources: {}", stats.emotion_count),
            format!("Updates: {}", self.grid.update_count),
            format!("FPS: {:.0}", get_fps()),
            format!("Status: {}", if self.paused { "PAUSED" } else { "RUNNING" }),
            "".to_string(),
            "Controls:".to_string(),
            "SPACE - Pause/Resume".to_string(),
            "R - Randomize".to_string(),
            "C - Clear".to_string(),
            "H - Toggle Help".to_string(),
            "ESC - Exit".to_string(),
        ];

        let mut content_y = y + 50.0;
        for line in &content {
            draw_text(line, x + 10.0, content_y, 16.0, text_primary);
            content_y += 18.0;
        }
    }

    fn render_rules_panel(&self) {
        let panel_bg = Color::new(0.23, 0.26, 0.32, 1.0);  // #3b4252
        let panel_border = Color::new(0.26, 0.30, 0.37, 1.0); // #434c5e
        let accent = Color::new(0.53, 0.75, 0.82, 1.0); // #88c0d0
        let text_primary = Color::new(0.90, 0.91, 0.94, 1.0); // #e5e9f0

        let width = 300.0;
        let height = 250.0;
        let x = 20.0;
        let y = screen_height() - height - 20.0;

        // Panel background
        draw_rectangle(x, y, width, height, panel_bg);
        draw_rectangle_lines(x, y, width, height, 2.0, panel_border);

        // Title
        draw_text("📋 Rules", x + 10.0, y + 30.0, 20.0, accent);

        // Content
        let content = [
            "Conway's Game of Life Rules:",
            "",
            "• Any live cell with < 2 neighbors dies",
            "• Any live cell with 2-3 neighbors lives",
            "• Any live cell with > 3 neighbors dies",
            "• Any dead cell with exactly 3 neighbors",
            "  becomes alive",
            "",
            "Emotion Spreading:",
            "• Live cells inherit emotions from",
            "  neighboring cells",
            "• Emotions decay over time",
            "• Multiple emotions compete for",
            "  dominance"
        ];

        let mut content_y = y + 50.0;
        for line in &content {
            draw_text(line, x + 10.0, content_y, 14.0, text_primary);
            content_y += 16.0;
        }
    }

    fn render_help_overlay(&self) {
        // Semi-transparent overlay
        let overlay_color = Color::new(0.18, 0.20, 0.25, 0.8); // #2e3440 with alpha
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), overlay_color);

        let panel_bg = Color::new(0.23, 0.26, 0.32, 1.0);  // #3b4252
        let accent = Color::new(0.53, 0.75, 0.82, 1.0); // #88c0d0
        let text_primary = Color::new(0.90, 0.91, 0.94, 1.0); // #e5e9f0

        // Help panel
        let help_width = 600.0;
        let help_height = 500.0;
        let help_x = (screen_width() - help_width) / 2.0;
        let help_y = (screen_height() - help_height) / 2.0;

        draw_rectangle(help_x, help_y, help_width, help_height, panel_bg);
        draw_rectangle_lines(help_x, help_y, help_width, help_height, 3.0, accent);

        // Help title
        draw_text("❓ HELP", help_x + 20.0, help_y + 40.0, 32.0, accent);

        // Help content
        let help_content = [
            "🎮 EMOTION CONTAGION - CONWAY'S GAME OF LIFE",
            "",
            "HOW TO PLAY:",
            "1. Click and drag characters from the toolbox",
            "2. Drop them anywhere on the grid",
            "3. Watch emotions spread through the cellular automata",
            "",
            "CONTROLS:",
            "• SPACE - Pause/Resume simulation",
            "• R - Randomize the grid",
            "• C - Clear the grid",
            "• H - Toggle this help screen",
            "• ESC - Exit the game",
            "",
            "EMOTIONS:",
            "• Each character represents an emotion",
            "• Emotions spread to neighboring cells",
            "• Multiple emotions compete for dominance",
            "• Emotions decay over time",
            "",
            "CONWAY'S RULES:",
            "• Live cells with 2-3 neighbors survive",
            "• Dead cells with exactly 3 neighbors become alive",
            "• All other cells die or stay dead"
        ];

        let mut content_y = help_y + 80.0;
        for line in &help_content {
            if content_y + 20.0 < help_y + help_height {
                draw_text(line, help_x + 20.0, content_y, 16.0, text_primary);
                content_y += 20.0;
            }
        }
    }
}

#[macroquad::main("🌊 Emotion Contagion - Conway's Game of Life")]
async fn main() {
    println!("🌊 Emotion Contagion - Conway's Game of Life");
    println!("Built as a desktop version of the Python/HTML implementations!");
    println!("");
    println!("🎮 CONTROLS:");
    println!("   🖱️ Click and drag: Add emotion sources");
    println!("   ⏯️ SPACE: Pause/Resume");
    println!("   🎲 R: Randomize grid");
    println!("   🗑️ C: Clear grid");
    println!("   ❓ H: Toggle Help");
    println!("   🚪 ESC: Exit");
    println!("");
    
    let mut game_state = GameState::new();
    
    // Add some initial emotion sources like the Python version
    game_state.grid.add_emotion_source(EmotionType::Joy, 100.0, 100.0);
    game_state.grid.add_emotion_source(EmotionType::Sadness, 300.0, 100.0);
    game_state.grid.add_emotion_source(EmotionType::Anger, 200.0, 200.0);
    
    println!("🚀 Game loop starting...");
    
    loop {
        let dt = get_frame_time();
        
        game_state.handle_input();
        game_state.update(dt);
        game_state.render();
        
        next_frame().await;
    }
}