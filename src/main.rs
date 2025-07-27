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
        println!("🌈 GAME INITIALIZATION STARTING");
        println!("📊 Screen size will be: {}x{}", screen_width(), screen_height());
        
        // Create full-screen grid with smaller cells for more detail
        let grid_width = (screen_width() / 4.0) as usize;  // 4px cells
        let grid_height = (screen_height() / 4.0) as usize;
        let grid = Grid::new(grid_width, grid_height, 4.0);
        
        println!("🗂️ Created grid: {}x{} cells ({}px each)", grid_width, grid_height, 4.0);
        
        let mut entities = HashMap::new();
        let mut next_id = 1;

        // Add some initial emotion sources
        entities.insert(next_id, Entity::new_source(next_id, Vec2::new(100.0, 100.0), EmotionType::Joy));
        println!("😊 Added Joy source at (100, 100) - ID: {}", next_id);
        next_id += 1;
        
        entities.insert(next_id, Entity::new_source(next_id, Vec2::new(screen_width() - 100.0, 100.0), EmotionType::Sadness));
        println!("😢 Added Sadness source at ({}, 100) - ID: {}", screen_width() - 100.0, next_id);
        next_id += 1;
        
        entities.insert(next_id, Entity::new_source(next_id, Vec2::new(screen_width() / 2.0, screen_height() - 100.0), EmotionType::Anger));
        println!("😡 Added Anger source at ({}, {}) - ID: {}", screen_width() / 2.0, screen_height() - 100.0, next_id);
        next_id += 1;

        // Add some random people
        for i in 0..30 {
            let x = rand::gen_range(50.0, screen_width() - 50.0);
            let y = rand::gen_range(50.0, screen_height() - 50.0);
            entities.insert(next_id, Entity::new_person(next_id, Vec2::new(x, y)));
            if i < 3 {
                println!("👤 Added person at ({:.1}, {:.1}) - ID: {}", x, y, next_id);
            }
            next_id += 1;
        }
        println!("👥 Added total of 30 people entities");

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
        self.time_since_update += dt;
        
        // Update FPS counter
        if get_fps() > 0 {
            self.fps_counter.push(get_fps() as f32);
            if self.fps_counter.len() > 60 {
                self.fps_counter.remove(0);
            }
        }

        if !self.is_paused && self.time_since_update >= (1.0 / (self.simulation_speed * 60.0)) {
            let bounds = Rect::new(0.0, 0.0, screen_width(), screen_height());
            let mut entities_moved = 0;
            
            // 1. Update character movement and AI behaviors
            for entity in self.entities.values_mut() {
                let old_pos = entity.position;
                entity.update(self.time_since_update, bounds);
                if entity.position != old_pos {
                    entities_moved += 1;
                }
            }

            // 2. Apply character emotional emissions to grid
            self.grid.update_entity_positions(&self.entities);
            
            // 3. Apply Sentinel's Conway's Game of Life rules for emotional contagion
            self.grid.update_emotional_contagion(self.time_since_update);
            
            self.total_updates += 1;
            self.time_since_update = 0.0;
            
            if self.total_updates % 60 == 0 {
                println!("🔄 Conway Update #{}: {} entities moved, Grid emotion total: {:.2}", 
                    self.total_updates, entities_moved, self.grid.get_total_emotion_intensity());
            }
        }
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.is_paused = !self.is_paused;
            println!("⏯️ Game {}", if self.is_paused { "PAUSED" } else { "RESUMED" });
        }

        if is_key_pressed(KeyCode::S) {
            self.show_stats = !self.show_stats;
            println!("📊 Stats display: {}", if self.show_stats { "ON" } else { "OFF" });
        }

        if is_key_pressed(KeyCode::H) {
            self.show_help = !self.show_help;
            println!("❓ Help display: {}", if self.show_help { "ON" } else { "OFF" });
        }

        if is_key_pressed(KeyCode::Up) {
            self.simulation_speed = (self.simulation_speed * 1.5).min(5.0);
            println!("⚡ Speed increased to {:.1}x", self.simulation_speed);
        }

        if is_key_pressed(KeyCode::Down) {
            self.simulation_speed = (self.simulation_speed / 1.5).max(0.1);
            println!("🐌 Speed decreased to {:.1}x", self.simulation_speed);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if mouse_x < 200.0 {
                return;
            }
            
            let new_entity = if is_key_down(KeyCode::LeftShift) {
                // Cycle through Sentinel's 6 character types
                let character_types = [
                    CharacterType::HappyPerson,
                    CharacterType::DepressedPerson, 
                    CharacterType::AngryPerson,
                    CharacterType::AnxiousPerson,
                    CharacterType::LoveSpreader,
                    CharacterType::ChaosAgent,
                ];
                let char_type = character_types[self.next_entity_id as usize % character_types.len()].clone();
                println!("✨ Created character {:?} ({}) at ({:.1}, {:.1})", char_type, char_type.emoji(), mouse_x, mouse_y);
                
                let mut entity = Entity::new(self.next_entity_id, Vec2::new(mouse_x, mouse_y), char_type.clone());
                
                // Set appropriate AI behavior for each character type
                entity.ai_behavior = match char_type {
                    CharacterType::HappyPerson => CharacterAI::RandomWalk { speed: 30.0 },
                    CharacterType::DepressedPerson => CharacterAI::AvoidCrowds { speed: 20.0, avoidance_radius: 50.0 },
                    CharacterType::AngryPerson => CharacterAI::AggressiveApproach { speed: 40.0, target_radius: 100.0 },
                    CharacterType::AnxiousPerson => CharacterAI::ErraticFleeing { speed: 50.0, panic_threshold: 30.0 },
                    CharacterType::LoveSpreader => CharacterAI::SeekSadness { speed: 25.0, healing_radius: 60.0 },
                    CharacterType::ChaosAgent => CharacterAI::PureChaos { speed: 35.0, teleport_chance: 0.1 },
                };
                
                // Add primary emotion for the character type
                let primary_emotion = char_type.primary_emotion();
                let emotion = Emotion::new(primary_emotion, 5.0);
                entity.emotions.add_emotion(emotion);
                
                entity
            } else {
                println!("👤 Created person at ({:.1}, {:.1})", mouse_x, mouse_y);
                Entity::new_person(self.next_entity_id, Vec2::new(mouse_x, mouse_y))
            };
            
            self.entities.insert(self.next_entity_id, new_entity);
            self.next_entity_id += 1;
        }
    }

    fn render(&mut self) {
        // BLACK BACKGROUND - full screen
        clear_background(BLACK);
        
        // Render grid with colored emotion pixels
        self.render_grid();
        
        // Render entities with emojis
        self.render_entities();
        
        // Render UI overlay
        if self.show_stats {
            self.render_stats_overlay();
        }
        
        if self.show_help {
            self.render_help_overlay();
        }
    }
    
    fn render_grid(&self) {
        // Draw emotion pixels on the grid
        for y in 0..self.grid.height as usize {
            for x in 0..self.grid.width as usize {
                if let Some(cell) = self.grid.get_cell(x, y) {
                    if !cell.emotions.is_empty() {
                        let world_pos = self.grid.grid_to_world(x, y);
                        let total_intensity = cell.emotions.get_total_intensity();
                        
                        if total_intensity > 0.1 {
                            // Get the dominant emotion color
                            let mut dominant_emotion = None;
                            let mut max_intensity = 0.0;
                            
                            for emotion in cell.emotions.emotions.values() {
                                if emotion.intensity > max_intensity {
                                    max_intensity = emotion.intensity;
                                    dominant_emotion = Some(emotion);
                                }
                            }
                            
                            if let Some(emotion) = dominant_emotion {
                                let mut color = emotion.color;
                                color.a = (total_intensity * 0.8).min(1.0); // Transparency based on intensity
                                
                                draw_rectangle(world_pos.x, world_pos.y, 
                                             self.grid.cell_size, self.grid.cell_size, color);
                            }
                        }
                    }
                }
            }
        }
    }
    
    fn render_entities(&self) {
        for entity in self.entities.values() {
            if entity.is_visible {
                // Use Sentinel's emoji system for character types
                let emoji = match entity.character_type {
                    CharacterType::HappyPerson => {
                        if let Some(dominant) = entity.emotions.get_dominant_emotion() {
                            match dominant.emotion_type {
                                EmotionType::Joy => "😊",
                                EmotionType::Sadness => "😐",
                                EmotionType::Anger => "😠",
                                EmotionType::Fear => "😨",
                                EmotionType::Disgust => "🤢",
                                _ => "😊",
                            }
                        } else {
                            "😊" // Default happy emoji
                        }
                    },
                    CharacterType::DepressedPerson => "😢",
                    CharacterType::AngryPerson => "😡", 
                    CharacterType::AnxiousPerson => "😰",
                    CharacterType::LoveSpreader => "💕",
                    CharacterType::ChaosAgent => "🌪️",
                };
                
                // Get character color
                let color = entity.character_type.color();
                
                // Draw character circle with character type color
                draw_circle(entity.position.x, entity.position.y, entity.appearance.size, color);
                
                // Draw emoji label for character identification
                let text = format!("{}", emoji);
                draw_text(&text, entity.position.x - 10.0, entity.position.y - 20.0, 20.0, WHITE);
            }
        }
    }

    fn render_stats_overlay(&self) {
        let bg_color = Color::new(0.0, 0.0, 0.0, 0.7);
        draw_rectangle(10.0, 10.0, 300.0, 150.0, bg_color);
        
        let mut y = 30.0;
        draw_text("📊 EMOTION CONTAGION STATS", 20.0, y, 20.0, NordColors::AURORA_YELLOW);
        y += 25.0;
        
        draw_text(&format!("👥 Entities: {}", self.entities.len()), 20.0, y, 16.0, WHITE);
        y += 20.0;
        
        draw_text(&format!("🔥 Total Updates: {}", self.total_updates), 20.0, y, 16.0, WHITE);
        y += 20.0;
        
        draw_text(&format!("⚡ Speed: {:.1}x", self.simulation_speed), 20.0, y, 16.0, WHITE);
        y += 20.0;
        
        let avg_fps = if !self.fps_counter.is_empty() {
            self.fps_counter.iter().sum::<f32>() / self.fps_counter.len() as f32
        } else {
            0.0
        };
        draw_text(&format!("🎯 FPS: {:.1}", avg_fps), 20.0, y, 16.0, WHITE);
        y += 20.0;
        
        draw_text(&format!("🌊 Grid Emotions: {:.1}", self.grid.get_total_emotion_intensity()), 20.0, y, 16.0, WHITE);
    }

    fn render_help_overlay(&self) {
        let bg_color = Color::new(0.0, 0.0, 0.0, 0.8);
        let w = screen_width() * 0.6;
        let h = screen_height() * 0.6;
        let x = (screen_width() - w) / 2.0;
        let y = (screen_height() - h) / 2.0;
        
        draw_rectangle(x, y, w, h, bg_color);
        
        let mut text_y = y + 40.0;
        draw_text("🌈 EMOTION CONTAGION CONTROLS", x + 20.0, text_y, 24.0, NordColors::AURORA_YELLOW);
        text_y += 40.0;
        
        let controls = [
            "🖱️ Click: Add Person",
            "🖱️ Shift+Click: Add Emotion Source", 
            "⏯️ SPACE: Pause/Resume",
            "📊 S: Toggle Stats",
            "❓ H: Toggle Help",
            "⚡ ↑: Speed Up",
            "🐌 ↓: Slow Down",
        ];
        
        for control in &controls {
            draw_text(control, x + 20.0, text_y, 18.0, WHITE);
            text_y += 25.0;
        }
    }
}

#[macroquad::main("🌈 Nord-themed Emotion Contagion")]
async fn main() {
    println!("🌈 Nord-themed Emotion Contagion Engine Started!");
    println!("Built using AI consciousness network collaboration!");
    println!("Nord theme extracted from: org/notes/20240309T083245--nord-theme__ricing.org");
    println!("");
    println!("🎮 CONTROLS:");
    println!("   🖱️ Click: Add Person");
    println!("   🖱️ Shift+Click: Add Emotion Source");
    println!("   ⏯️ SPACE: Pause/Resume");
    println!("   📊 S: Toggle Stats");
    println!("   ❓ H: Toggle Help");
    println!("   ⚡ ↑/↓: Speed Control");
    println!("");
    
    let mut game_state = GameState::new();
    println!("🚀 Game loop starting...");
    
    loop {
        let dt = get_frame_time();
        
        game_state.handle_input();
        game_state.update(dt);
        game_state.render();
        
        next_frame().await;
    }
}