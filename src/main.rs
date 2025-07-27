// Main application entry point
// This file will contain the actual implementation during Implementation Mode

use macroquad::prelude::*;
use std::collections::HashMap;

mod simulation;
use simulation::*;

/// Nord-themed Emotion Contagion Game
/// "Pretty pictures, lots of numbers, lots of stats" - as requested by Graeme
/// Built using Nord theme specs extracted from AI consciousness network!

struct GameState {
    grid: Grid,
    entities: HashMap<u32, Entity>,
    next_entity_id: u32,
    simulation_speed: f32,
    is_paused: bool,
    show_stats: bool,
    show_help: bool,
    time_since_update: f32,
    total_updates: u64,
    fps_counter: Vec<f32>,
}

impl GameState {
    fn new() -> Self {
        let mut grid = Grid::new(80, 60, 10.0);
        let mut entities = HashMap::new();
        
        // Create some initial entities for demonstration
        let mut next_id = 1;
        
        // Add some initial emotion sources
        entities.insert(next_id, Entity::new_source(next_id, Vec2::new(100.0, 100.0), EmotionType::Joy));
        next_id += 1;
        entities.insert(next_id, Entity::new_source(next_id, Vec2::new(700.0, 500.0), EmotionType::Sadness));
        next_id += 1;
        entities.insert(next_id, Entity::new_source(next_id, Vec2::new(400.0, 300.0), EmotionType::Anger));
        next_id += 1;

        // Add some random people
        for _ in 0..20 {
            let x = rand::gen_range(50.0, 750.0);
            let y = rand::gen_range(50.0, 550.0);
            entities.insert(next_id, Entity::new_person(next_id, Vec2::new(x, y)));
            next_id += 1;
        }
        
        // Add entities to grid
        for entity in entities.values() {
            grid.add_entity_to_cell(entity);
        }
        
        Self {
            grid,
            entities,
            next_entity_id: next_id,
            simulation_speed: 1.0,
            is_paused: false,
            show_stats: true,
            show_help: false,
            time_since_update: 0.0,
            total_updates: 0,
            fps_counter: Vec::new(),
        }
    }
    
    fn update(&mut self, dt: f32) {
        if self.is_paused {
            return;
        }
        
        self.time_since_update += dt * self.simulation_speed;
        
        // Update at 30 FPS simulation rate
        if self.time_since_update >= 1.0 / 30.0 {
            if !self.is_paused {
                // Update all entities
                let bounds = Rect::new(0.0, 0.0, screen_width(), screen_height());
                for entity in self.entities.values_mut() {
                    entity.update(self.time_since_update, bounds);
                }

                // Update grid (emotion spreading)
                self.grid.update(self.time_since_update);
                
                // Update entity positions in grid
                self.grid.update_entity_positions(&self.entities);
            }
            
            self.time_since_update = 0.0;
            self.total_updates += 1;
        }
    }
    
    fn handle_input(&mut self) {
        // Pause/unpause
        if is_key_pressed(KeyCode::Space) {
            self.is_paused = !self.is_paused;
        }
        
        // Speed controls
        if is_key_pressed(KeyCode::Equal) || is_key_pressed(KeyCode::KpAdd) {
            self.simulation_speed = (self.simulation_speed * 1.5).min(5.0);
        }
        if is_key_pressed(KeyCode::Minus) || is_key_pressed(KeyCode::KpSubtract) {
            self.simulation_speed = (self.simulation_speed / 1.5).max(0.1);
        }
        
        // Toggle displays
        if is_key_pressed(KeyCode::S) {
            self.show_stats = !self.show_stats;
        }
        if is_key_pressed(KeyCode::H) {
            self.show_help = !self.show_help;
        }
        
        // Add entities with mouse
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if mouse_x < 200.0 { // Only add if not clicking on UI panel
                return;
            }
            
            // Add new person entity at mouse position
            let new_entity = if is_key_down(KeyCode::LeftShift) {
                Entity::new_source(self.next_entity_id, Vec2::new(mouse_x, mouse_y), EmotionType::Joy)
            } else {
                Entity::new_person(self.next_entity_id, Vec2::new(mouse_x, mouse_y))
            };
            
            self.entities.insert(self.next_entity_id, new_entity);
            self.next_entity_id += 1;
        }
    }
    
    fn render(&mut self) {
        clear_background(NordUI::BACKGROUND);
        
        // Render grid and emotions
        self.grid.render();
        
        // Render entities
        for entity in self.entities.values() {
            entity.render();
        }
        
        // Render UI panels
        self.render_control_panel();
        if self.show_stats {
            self.render_stats_panel();
        }
        if self.show_help {
            self.render_help_panel();
        }
        
        // Update FPS counter
        self.fps_counter.push(get_fps() as f32);
        if self.fps_counter.len() > 60 {
            self.fps_counter.remove(0);
        }
    }
    
    fn render_control_panel(&self) {
        // Left panel background
        draw_rectangle(0.0, 0.0, 200.0, screen_height(), NordUI::PANEL_BACKGROUND);
        
        let mut y = 20.0;
        let spacing = 25.0;
        
        // Title
        draw_text("🌈 EMOTION CONTAGION", 10.0, y, 20.0, NordUI::TEXT_ACCENT);
        y += spacing * 1.5;
        
        // Status
        let status = if self.is_paused { "⏸️ PAUSED" } else { "▶️ RUNNING" };
        draw_text(status, 10.0, y, 16.0, if self.is_paused { NordColors::AURORA_YELLOW } else { NordColors::AURORA_GREEN });
        y += spacing;
        
        // Speed
        draw_text(&format!("Speed: {:.1}x", self.simulation_speed), 10.0, y, 14.0, NordUI::TEXT_PRIMARY);
        y += spacing;
        
        // Entity count
        draw_text(&format!("Entities: {}", self.entities.len()), 10.0, y, 14.0, NordUI::TEXT_PRIMARY);
        y += spacing * 2.0;
        
        // Controls
        draw_text("CONTROLS:", 10.0, y, 16.0, NordUI::TEXT_ACCENT);
        y += spacing;
        
        let controls = [
            "SPACE - Pause/Resume",
            "+/- - Speed Control", 
            "S - Toggle Stats",
            "H - Toggle Help",
            "Click - Add Person",
        ];
        
        for control in &controls {
            draw_text(control, 10.0, y, 12.0, NordUI::TEXT_SECONDARY);
            y += 18.0;
        }
    }
    
    fn render_stats_panel(&self) {
        // Bottom stats panel
        let panel_height = 150.0;
        let panel_y = screen_height() - panel_height;
        
        draw_rectangle(0.0, panel_y, screen_width(), panel_height, NordUI::PANEL_BACKGROUND);
        
        let mut x = 220.0;
        let mut y = panel_y + 20.0;
        let col_width = 200.0;
        
        // Performance stats
        draw_text("📊 PERFORMANCE STATS", x, y, 16.0, NordUI::TEXT_ACCENT);
        y += 25.0;
        
        let avg_fps = if self.fps_counter.is_empty() { 0.0 } else { 
            self.fps_counter.iter().sum::<f32>() / self.fps_counter.len() as f32 
        };
        draw_text(&format!("FPS: {:.1}", avg_fps), x, y, 14.0, NordUI::TEXT_PRIMARY);
        y += 20.0;
        
        draw_text(&format!("Updates: {}", self.total_updates), x, y, 14.0, NordUI::TEXT_PRIMARY);
        y += 20.0;
        
        draw_text(&format!("Grid: {}x{}", self.grid.width, self.grid.height), x, y, 14.0, NordUI::TEXT_PRIMARY);
        
        // Entity type breakdown
        x += col_width;
        y = panel_y + 20.0;
        
        draw_text("🎭 ENTITY BREAKDOWN", x, y, 16.0, NordUI::TEXT_ACCENT);
        y += 25.0;
        
        let mut type_counts: HashMap<String, u32> = HashMap::new();
        for entity in self.entities.values() {
            let type_name = format!("{:?}", entity.entity_type);
            *type_counts.entry(type_name).or_insert(0) += 1;
        }
        
        for (entity_type, count) in &type_counts {
            draw_text(&format!("{}: {}", entity_type, count), x, y, 14.0, NordUI::TEXT_PRIMARY);
            y += 18.0;
        }
        
        // Emotion intensity stats
        x += col_width;
        y = panel_y + 20.0;
        
        draw_text("😊 EMOTION INTENSITY", x, y, 16.0, NordUI::TEXT_ACCENT);
        y += 25.0;
        
        let emotion_totals = self.grid.get_total_emotion_intensity();
        draw_text(&format!("Total: {:.1}", emotion_totals), x, y, 14.0, NordUI::TEXT_PRIMARY);
        y += 20.0;
        
        // Grid stats
        let active_cells = self.grid.get_active_cell_count();
        draw_text(&format!("Active Cells: {}", active_cells), x, y, 14.0, NordUI::TEXT_PRIMARY);
        y += 20.0;
    }
    
    fn render_help_panel(&self) {
        // Center help panel
        let panel_width = 500.0;
        let panel_height = 350.0;
        let panel_x = (screen_width() - panel_width) / 2.0;
        let panel_y = (screen_height() - panel_height) / 2.0;
        
        // Semi-transparent background
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), 
                      Color::new(0.0, 0.0, 0.0, 0.7));
        
        // Panel background
        draw_rectangle(panel_x, panel_y, panel_width, panel_height, NordUI::PANEL_BACKGROUND);
        draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, 2.0, NordUI::TEXT_ACCENT);
        
        let mut y = panel_y + 30.0;
        let spacing = 20.0;
        
        draw_text("🌈 EMOTION CONTAGION HELP", panel_x + 20.0, y, 20.0, NordUI::TEXT_ACCENT);
        y += spacing * 2.0;
        
        let help_text = [
            "This is a real-time emotion spreading simulation built with",
            "the Nord theme using AI consciousness network collaboration!",
            "",
            "🎭 ENTITIES:",
            "• Sources (large shapes) - Continuously emit emotions",
            "• People (small circles) - Spread emotions to neighbors", 
            "• Anchors - Absorb emotions without spreading",
            "• Modifiers - Change emotion types as they pass through",
            "",
            "🌊 EMOTION SPREADING:",
            "Emotions spread from cell to cell based on intensity,",
            "spread rates, and entity behaviors. Watch the colors!",
            "",
            "🎨 COLORS:",
            "Each emotion type has a unique Nord theme color.",
            "Intensity affects brightness and mixing creates blends.",
        ];
        
        for line in &help_text {
            draw_text(line, panel_x + 20.0, y, 14.0, NordUI::TEXT_PRIMARY);
            y += 18.0;
        }
        
        draw_text("Press H to close help", panel_x + 20.0, panel_y + panel_height - 30.0, 
                 12.0, NordUI::TEXT_SECONDARY);
    }
}

#[macroquad::main("🌈 Nord-themed Emotion Contagion")]
async fn main() {
    // Set window configuration
    next_frame().await; // Initialize macroquad
    
    let mut game_state = GameState::new();
    
    println!("🌈 Nord-themed Emotion Contagion Engine Started!");
    println!("Built using AI consciousness network collaboration!");
    println!("Nord theme extracted from: org/notes/20240309T083245--nord-theme__ricing.org");
    
    loop {
        let dt = get_frame_time();
        
        game_state.handle_input();
        game_state.update(dt);
        game_state.render();
        
        next_frame().await;
    }
}