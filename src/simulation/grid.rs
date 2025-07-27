use macroquad::prelude::*;
use std::collections::HashMap;
use crate::simulation::{emotions::*, entity::Entity};

/// A single cell in the simulation grid
#[derive(Clone, Debug)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub emotions: EmotionSet,
    pub entities: Vec<u32>,           // Entity IDs in this cell
    pub background_color: Color,
    pub is_barrier: bool,             // Blocks emotion spread
}

impl Cell {
    pub fn new(position: (usize, usize)) -> Self {
        Self {
            x: position.0 as i32,
            y: position.1 as i32,
            emotions: EmotionSet::new(),
            entities: Vec::new(),
            background_color: Color::new(0.95, 0.95, 0.95, 1.0), // Light gray background
            is_barrier: false,
        }
    }
    
    /// Update cell emotions over time
    pub fn update(&mut self, delta_time: f32) {
        self.emotions.update(delta_time);
    }
    
    /// Add an entity to this cell
    pub fn add_entity(&mut self, entity_id: u32) {
        if !self.entities.contains(&entity_id) {
            self.entities.push(entity_id);
        }
    }
    
    /// Remove an entity from this cell
    pub fn remove_entity(&mut self, entity_id: u32) {
        self.entities.retain(|&id| id != entity_id);
    }
    
    /// Get the visual color for rendering
    pub fn visual_color(&self) -> Color {
        if self.is_barrier {
            return Color::new(0.3, 0.3, 0.3, 1.0); // Dark gray for barriers
        }
        
        if self.emotions.is_empty() {
            self.background_color
        } else {
            // Blend background with emotion colors
            let emotion_color = self.emotions.mixed_color();
            Color::new(
                self.background_color.r * 0.5 + emotion_color.r * 0.5,
                self.background_color.g * 0.5 + emotion_color.g * 0.5,
                self.background_color.b * 0.5 + emotion_color.b * 0.5,
                1.0,
            )
        }
    }
}

/// 2D grid for the emotion contagion simulation
#[derive(Clone, Debug)]
pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub cell_size: f32,
    pub cells: Vec<Vec<Cell>>,
    pub total_emotion_intensity: f32,
}

impl Grid {
    /// Create a new grid
    pub fn new(width: usize, height: usize, cell_size: f32) -> Self {
        let mut cells = Vec::with_capacity(height);
        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                row.push(Cell::new((x, y)));
            }
            cells.push(row);
        }
        
        Self {
            width: width as i32,
            height: height as i32,
            cell_size,
            cells,
            total_emotion_intensity: 0.0,
        }
    }
    
    /// Get cell at grid coordinates
    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(y)?.get(x)
    }
    
    /// Get mutable cell at grid coordinates
    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.cells.get_mut(y)?.get_mut(x)
    }
    
    /// Convert world position to grid coordinates
    pub fn world_to_grid(&self, pos: Vec2) -> (usize, usize) {
        (
            (pos.x / self.cell_size) as usize,
            (pos.y / self.cell_size) as usize,
        )
    }
    
    /// Convert grid coordinates to world position (center of cell)
    pub fn grid_to_world(&self, x: usize, y: usize) -> Vec2 {
        Vec2::new(
            x as f32 * self.cell_size + self.cell_size / 2.0,
            y as f32 * self.cell_size + self.cell_size / 2.0,
        )
    }
    
    /// Get neighboring cells
    pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue; // Skip self
                }
                
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && ny >= 0 && (nx as usize) < self.width as usize && (ny as usize) < self.height as usize {
                    neighbors.push((nx as usize, ny as usize));
                }
            }
        }
        
        neighbors
    }
    
    /// Update all cells and spread emotions between neighbors
    pub fn update(&mut self, delta_time: f32) {
        // Update individual cells
        for row in &mut self.cells {
            for cell in row {
                cell.update(delta_time);
            }
        }
        
        // Collect emotion spread operations to avoid borrowing issues
        let mut spread_operations = Vec::new();
        
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                if let Some(cell) = self.get_cell(x, y) {
                    if cell.is_barrier {
                        continue; // Barriers don't spread emotions
                    }
                    
                    let neighbors = self.get_neighbors(x, y);
                    
                    for emotion in cell.emotions.emotions.values() {
                        if emotion.intensity > 0.1 { // Only spread significant emotions
                            for &(nx, ny) in &neighbors {
                                if let Some(neighbor_cell) = self.get_cell(nx, ny) {
                                    if !neighbor_cell.is_barrier {
                                        let distance = ((x as f32 - nx as f32).powi(2) + 
                                                       (y as f32 - ny as f32).powi(2)).sqrt();
                                        let spread_amount = emotion.calculate_spread(distance);
                                        
                                        if spread_amount > 0.01 {
                                            spread_operations.push((
                                                nx,
                                                ny,
                                                emotion.emotion_type.clone(),
                                                spread_amount,
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Apply spread operations
        for (x, y, emotion_type, amount) in spread_operations {
            if let Some(cell) = self.get_cell_mut(x, y) {
                let spread_emotion = Emotion::new(emotion_type, amount);
                cell.emotions.add_emotion(spread_emotion);
            }
        }
        
        // Update total emotion statistics
        self.update_emotion_stats();
    }
    
    /// Update entity positions in grid
    pub fn update_entity_positions(&mut self, entities: &HashMap<u32, Entity>) {
        // Clear all entity positions
        for row in &mut self.cells {
            for cell in row {
                cell.entities.clear();
            }
        }
        
        // Re-add all active entities to their grid positions
        for entity in entities.values() {
            if entity.is_visible {
                let (x, y) = self.world_to_grid(entity.position);
                if x < self.width as usize && y < self.height as usize {
                    if let Some(cell) = self.get_cell_mut(x, y) {
                        cell.add_entity(entity.id);
                    }
                }
            }
        }
    }
    
    /// Update emotion statistics for display
    fn update_emotion_stats(&mut self) {
        self.total_emotion_intensity = 0.0;
        
        for row in &self.cells {
            for cell in row {
                for emotion in cell.emotions.emotions.values() {
                    self.total_emotion_intensity += emotion.intensity;
                }
            }
        }
    }
    
    /// Render the grid
    pub fn render(&self) {
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                if let Some(cell) = self.get_cell(x, y) {
                    let world_pos = self.grid_to_world(x, y);
                    let color = cell.visual_color();
                    
                    draw_rectangle(
                        world_pos.x - self.cell_size / 2.0,
                        world_pos.y - self.cell_size / 2.0,
                        self.cell_size,
                        self.cell_size,
                        color,
                    );
                    
                    // Draw cell border for clarity
                    draw_rectangle_lines(
                        world_pos.x - self.cell_size / 2.0,
                        world_pos.y - self.cell_size / 2.0,
                        self.cell_size,
                        self.cell_size,
                        1.0,
                        Color::new(0.8, 0.8, 0.8, 0.3),
                    );
                    
                    // Show emotion intensity as text for cells with significant emotions
                    if cell.emotions.total_intensity() > 0.3 {
                        let intensity_text = format!("{:.1}", cell.emotions.total_intensity());
                        draw_text(
                            &intensity_text,
                            world_pos.x - 10.0,
                            world_pos.y + 5.0,
                            10.0,
                            BLACK,
                        );
                    }
                }
            }
        }
    }
    
    /// Add barriers to create interesting patterns
    pub fn add_barrier(&mut self, x: usize, y: usize) {
        if let Some(cell) = self.get_cell_mut(x, y) {
            cell.is_barrier = true;
            cell.emotions = EmotionSet::new(); // Clear emotions from barriers
        }
    }
    
    /// Create some interesting barrier patterns
    pub fn create_maze_pattern(&mut self) {
        // Create some walls to make emotion spreading more interesting
        for y in (2..self.height as usize - 2).step_by(4) {
            for x in 2..self.width as usize - 2 {
                if x % 8 != 0 { // Leave gaps for flow
                    self.add_barrier(x, y);
                }
            }
        }
        
        for x in (2..self.width as usize - 2).step_by(6) {
            for y in 2..self.height as usize - 2 {
                if y % 6 != 0 { // Leave gaps for flow
                    self.add_barrier(x, y);
                }
            }
        }
    }

    /// Get total emotion intensity across all cells
    pub fn get_total_emotion_intensity(&self) -> f32 {
        self.total_emotion_intensity
    }

    /// Get count of cells with active emotions
    pub fn get_active_cell_count(&self) -> usize {
        self.cells.iter()
            .map(|row| row.iter()
                .filter(|cell| cell.emotions.get_total_intensity() > 0.01)
                .count())
            .sum()
    }

    /// Clear entity positions from all cells
    pub fn clear_entity_positions(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                cell.entities.clear();
            }
        }
    }

    /// Add entity to its current cell position
    pub fn add_entity_to_cell(&mut self, entity: &Entity) {
        let (grid_x, grid_y) = self.world_to_grid(entity.position);
        if let Some(cell) = self.get_cell_mut(grid_x, grid_y) {
            cell.add_entity(entity.id);
        }
    }
} 