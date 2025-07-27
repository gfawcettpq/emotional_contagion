use macroquad::prelude::*;
use crate::simulation::emotions::*;
use std::collections::HashMap;

/// Simple emotion grid - matches the Python version exactly
pub struct EmotionGrid {
    pub width: usize,
    pub height: usize,
    pub cell_size: f32,
    pub grid: Vec<Vec<EmotionCell>>,
    pub update_count: u32,
}

impl EmotionGrid {
    pub fn new(width: usize, height: usize, cell_size: f32) -> Self {
        let mut grid = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(EmotionCell::new());
            }
            grid.push(row);
        }

        Self {
            width,
            height,
            cell_size,
            grid,
            update_count: 0,
        }
    }

    /// Add an emotion source at world coordinates (like the Python version)
    pub fn add_emotion_source(&mut self, emotion: EmotionType, world_x: f32, world_y: f32) {
        let grid_x = (world_x / self.cell_size) as usize;
        let grid_y = (world_y / self.cell_size) as usize;
        
        if grid_x < self.width && grid_y < self.height {
            self.grid[grid_y][grid_x] = EmotionCell::new_with_emotion(emotion, 1.0);
            println!("😊 Added {} source at ({}, {}) -> grid({}, {})", 
                emotion.name(), world_x, world_y, grid_x, grid_y);
        }
    }

    /// Count live neighbors for Conway's Game of Life (exactly like Python)
    pub fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && ny >= 0 && 
                   (nx as usize) < self.width && 
                   (ny as usize) < self.height &&
                   self.grid[ny as usize][nx as usize].alive {
                    count += 1;
                }
            }
        }
        count
    }

    /// Get emotions from neighboring cells (like Python version)
    pub fn get_neighbor_emotions(&self, x: usize, y: usize) -> Vec<(EmotionType, f32)> {
        let mut emotions = Vec::new();
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && ny >= 0 && 
                   (nx as usize) < self.width && 
                   (ny as usize) < self.height {
                    let cell = &self.grid[ny as usize][nx as usize];
                    if cell.alive && cell.emotion.is_some() && cell.intensity > 0.1 {
                        emotions.push((cell.emotion.unwrap(), cell.intensity));
                    }
                }
            }
        }
        emotions
    }

    /// Find the dominant emotion from neighbors (like Python version)
    pub fn get_dominant_emotion(&self, emotions: &[(EmotionType, f32)]) -> Option<(EmotionType, f32)> {
        if emotions.is_empty() {
            return None;
        }

        let mut emotion_counts: HashMap<EmotionType, f32> = HashMap::new();
        
        for &(emotion, intensity) in emotions {
            *emotion_counts.entry(emotion).or_insert(0.0) += intensity;
        }

        emotion_counts.into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    /// Update the grid using Conway's Game of Life rules with emotion spreading
    /// This is EXACTLY like the Python version
    pub fn update(&mut self) -> GridStats {
        let mut new_grid = Vec::new();
        for _ in 0..self.height {
            let mut row = Vec::new();
            for _ in 0..self.width {
                row.push(EmotionCell::new());
            }
            new_grid.push(row);
        }

        let mut stats = GridStats::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.count_neighbors(x, y);
                let current_alive = self.grid[y][x].alive;
                
                // Conway's Game of Life rules - EXACTLY like the Python version
                let alive = if current_alive {
                    if neighbors < 2 || neighbors > 3 {
                        stats.deaths += 1;
                        false
                    } else {
                        true
                    }
                } else {
                    if neighbors == 3 {
                        stats.births += 1;
                        true
                    } else {
                        false
                    }
                };

                // Emotion spreading
                let mut emotion = self.grid[y][x].emotion;
                let mut intensity = self.grid[y][x].intensity * 0.9; // Decay

                if alive {
                    let neighbor_emotions = self.get_neighbor_emotions(x, y);
                    if !neighbor_emotions.is_empty() {
                        if let Some((dominant_emotion, dominant_intensity)) = self.get_dominant_emotion(&neighbor_emotions) {
                            emotion = Some(dominant_emotion);
                            intensity = (intensity + dominant_intensity * 0.3).min(1.0);
                            stats.emotion_spreads += 1;
                        }
                    }
                }

                new_grid[y][x] = EmotionCell {
                    alive,
                    emotion,
                    intensity,
                };

                if alive {
                    stats.alive_count += 1;
                    if emotion.is_some() && intensity > 0.1 {
                        stats.emotion_count += 1;
                    }
                }
            }
        }

        self.grid = new_grid;
        self.update_count += 1;

        if stats.births > 0 || stats.deaths > 0 || stats.emotion_spreads > 0 {
            println!("🔄 Conway Update #{}: {} births, {} deaths, {} emotion spreads", 
                self.update_count, stats.births, stats.deaths, stats.emotion_spreads);
        }

        stats
    }

    /// Render the grid with emotion colors
    pub fn render(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self.grid[y][x];
                if cell.alive {
                    let world_x = x as f32 * self.cell_size;
                    let world_y = y as f32 * self.cell_size;
                    let color = cell.display_color();
                    
                    if color.a > 0.0 { // Only draw if not transparent
                        draw_rectangle(world_x, world_y, self.cell_size, self.cell_size, color);
                    }
                }
            }
        }
    }

    /// Clear the entire grid
    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.grid[y][x] = EmotionCell::new();
            }
        }
        self.update_count = 0;
    }

    /// Randomize the grid with some emotion sources
    pub fn randomize(&mut self) {
        use ::rand::{Rng, thread_rng};
        let mut rng = thread_rng();
        
        // Clear first
        self.clear();
        
        // Create random alive cells (30% chance)
        for y in 0..self.height {
            for x in 0..self.width {
                if rng.gen_bool(0.3) {
                    self.grid[y][x].alive = true;
                }
            }
        }

        // Add some random emotion sources
        let emotions = EmotionType::all();
        for _ in 0..10 {
            let emotion = emotions[rng.gen_range(0..emotions.len())];
            let x = rng.gen_range(0.0..self.width as f32 * self.cell_size);
            let y = rng.gen_range(0.0..self.height as f32 * self.cell_size);
            self.add_emotion_source(emotion, x, y);
        }
    }

    /// Get statistics about the current grid state
    pub fn get_stats(&self) -> GridStats {
        let mut stats = GridStats::new();
        
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self.grid[y][x];
                if cell.alive {
                    stats.alive_count += 1;
                    if cell.emotion.is_some() && cell.intensity > 0.1 {
                        stats.emotion_count += 1;
                    }
                }
            }
        }
        
        stats
    }
}