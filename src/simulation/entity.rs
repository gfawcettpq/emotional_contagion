use serde::{Deserialize, Serialize};
use macroquad::prelude::*;
use std::collections::HashMap;
use super::emotions::{EmotionSet, EmotionType, Emotion};

/// Types of entities in the simulation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EntityType {
    Person,      // Regular entity that can give and receive emotions
    Source,      // Static emotion generator (never stops producing)
    Anchor,      // Fixed point with specific emotional properties
    Modifier,    // Affects nearby emotions (amplifier, dampener, etc.)
}

/// Movement patterns for entities
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MovementPattern {
    Static,                                           // Doesn't move
    Random { speed: f32, bounds: Option<Rect> },     // Random wandering
    Circular { center: Vec2, radius: f32, speed: f32 }, // Circular motion
    Follow { target_id: u32, distance: f32 },        // Follows another entity
}

/// Visual appearance configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Appearance {
    pub size: f32,
    pub shape: EntityShape,
    pub base_color: Color,
    pub show_emotions: bool,
    pub show_stats: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EntityShape {
    Circle,
    Square,
    Triangle,
    Heart,     // For love-focused entities
    Star,      // For joy-focused entities
}

/// Main entity in the simulation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: u32,
    pub position: Vec2,
    pub entity_type: EntityType,
    pub emotions: EmotionSet,
    pub movement: MovementPattern,
    pub appearance: Appearance,
    pub active: bool,
    
    // Movement state
    pub velocity: Vec2,
    pub movement_timer: f32,
    pub movement_angle: f32,
    
    // Stats for display
    pub total_emotions_given: f32,
    pub total_emotions_received: f32,
    pub lifetime: f32,
}

impl Entity {
    /// Create a new entity
    pub fn new(id: u32, position: Vec2, entity_type: EntityType) -> Self {
        let appearance = match entity_type {
            EntityType::Person => Appearance {
                size: 12.0,
                shape: EntityShape::Circle,
                base_color: WHITE,
                show_emotions: true,
                show_stats: false,
            },
            EntityType::Source => Appearance {
                size: 20.0,
                shape: EntityShape::Star,
                base_color: YELLOW,
                show_emotions: true,
                show_stats: true,
            },
            EntityType::Anchor => Appearance {
                size: 15.0,
                shape: EntityShape::Square,
                base_color: GRAY,
                show_emotions: true,
                show_stats: false,
            },
            EntityType::Modifier => Appearance {
                size: 18.0,
                shape: EntityShape::Triangle,
                base_color: PURPLE,
                show_emotions: false,
                show_stats: true,
            },
        };
        
        Self {
            id,
            position,
            entity_type,
            emotions: EmotionSet::new(),
            movement: MovementPattern::Static,
            appearance,
            active: true,
            velocity: Vec2::ZERO,
            movement_timer: 0.0,
            movement_angle: 0.0,
            total_emotions_given: 0.0,
            total_emotions_received: 0.0,
            lifetime: 0.0,
        }
    }
    
    /// Create a person entity with random movement
    pub fn person(id: u32, position: Vec2, speed: f32) -> Self {
        let mut entity = Self::new(id, position, EntityType::Person);
        entity.movement = MovementPattern::Random { 
            speed, 
            bounds: None 
        };
        entity
    }
    
    /// Create an emotion source that continuously produces emotions
    pub fn emotion_source(id: u32, position: Vec2, emotion_type: EmotionType, intensity: f32) -> Self {
        let mut entity = Self::new(id, position, EntityType::Source);
        entity.emotions.add_emotion(Emotion::new(emotion_type, intensity));
        entity
    }
    
    /// Update entity position and internal state
    pub fn update(&mut self, delta_time: f32, bounds: Rect) {
        self.lifetime += delta_time;
        self.movement_timer += delta_time;
        
        // Update emotions (decay over time)
        self.emotions.update(delta_time);
        
        // Update movement
        match &self.movement {
            MovementPattern::Static => {},
            
            MovementPattern::Random { speed, bounds: move_bounds } => {
                let bounds = move_bounds.unwrap_or(bounds);
                
                // Change direction occasionally
                if self.movement_timer > 2.0 {
                    self.movement_angle = rand::gen_range(0.0, 2.0 * std::f32::consts::PI);
                    self.movement_timer = 0.0;
                }
                
                // Move in current direction
                self.velocity = Vec2::new(
                    self.movement_angle.cos() * speed,
                    self.movement_angle.sin() * speed,
                );
                
                let new_pos = self.position + self.velocity * delta_time;
                
                // Bounce off boundaries
                let mut final_pos = new_pos;
                if new_pos.x < bounds.x || new_pos.x > bounds.x + bounds.w {
                    self.movement_angle = std::f32::consts::PI - self.movement_angle;
                    final_pos.x = new_pos.x.clamp(bounds.x, bounds.x + bounds.w);
                }
                if new_pos.y < bounds.y || new_pos.y > bounds.y + bounds.h {
                    self.movement_angle = -self.movement_angle;
                    final_pos.y = new_pos.y.clamp(bounds.y, bounds.y + bounds.h);
                }
                
                self.position = final_pos;
            },
            
            MovementPattern::Circular { center, radius, speed } => {
                self.movement_angle += speed * delta_time;
                self.position = *center + Vec2::new(
                    self.movement_angle.cos() * radius,
                    self.movement_angle.sin() * radius,
                );
            },
            
            MovementPattern::Follow { target_id: _, distance: _ } => {
                // TODO: Implement following behavior
            },
        }
        
        // Emotion sources continuously generate emotions
        if matches!(self.entity_type, EntityType::Source) {
            if let Some(dominant) = self.emotions.dominant_emotion() {
                let emotion_type = dominant.emotion_type.clone();
                let new_emotion = Emotion::new(emotion_type, 0.1); // Continuous low-level generation
                self.emotions.add_emotion(new_emotion);
            }
        }
    }
    
    /// Spread emotions to nearby entities
    pub fn spread_emotions_to(&mut self, other: &mut Entity, distance: f32) {
        if distance <= 0.0 || distance > 100.0 {
            return; // Too far away
        }
        
        let emotions_to_spread: Vec<_> = self.emotions.emotions.values().cloned().collect();
        
        for emotion in emotions_to_spread {
            let spread_amount = emotion.calculate_spread(distance);
            if spread_amount > 0.01 {
                let spread_emotion = Emotion::new(emotion.emotion_type.clone(), spread_amount);
                other.emotions.add_emotion(spread_emotion);
                other.total_emotions_received += spread_amount;
                self.total_emotions_given += spread_amount;
            }
        }
    }
    
    /// Render the entity
    pub fn render(&self) {
        if !self.active {
            return;
        }
        
        let color = if self.emotions.is_empty() {
            self.appearance.base_color
        } else {
            self.emotions.mixed_color()
        };
        
        // Draw entity shape
        match self.appearance.shape {
            EntityShape::Circle => {
                draw_circle(
                    self.position.x,
                    self.position.y,
                    self.appearance.size,
                    color,
                );
            },
            EntityShape::Square => {
                draw_rectangle(
                    self.position.x - self.appearance.size / 2.0,
                    self.position.y - self.appearance.size / 2.0,
                    self.appearance.size,
                    self.appearance.size,
                    color,
                );
            },
            EntityShape::Triangle => {
                // Simple triangle using lines
                let size = self.appearance.size;
                let x = self.position.x;
                let y = self.position.y;
                draw_triangle(
                    Vec2::new(x, y - size / 2.0),
                    Vec2::new(x - size / 2.0, y + size / 2.0),
                    Vec2::new(x + size / 2.0, y + size / 2.0),
                    color,
                );
            },
            EntityShape::Heart => {
                // Simplified heart as two circles and a triangle
                let size = self.appearance.size * 0.6;
                draw_circle(self.position.x - size / 3.0, self.position.y - size / 3.0, size / 2.0, color);
                draw_circle(self.position.x + size / 3.0, self.position.y - size / 3.0, size / 2.0, color);
                draw_triangle(
                    Vec2::new(self.position.x - size / 2.0, self.position.y),
                    Vec2::new(self.position.x + size / 2.0, self.position.y),
                    Vec2::new(self.position.x, self.position.y + size),
                    color,
                );
            },
            EntityShape::Star => {
                // Simple star as overlapping triangles
                let size = self.appearance.size;
                let x = self.position.x;
                let y = self.position.y;
                
                // Upward triangle
                draw_triangle(
                    Vec2::new(x, y - size / 2.0),
                    Vec2::new(x - size / 3.0, y + size / 4.0),
                    Vec2::new(x + size / 3.0, y + size / 4.0),
                    color,
                );
                
                // Downward triangle
                draw_triangle(
                    Vec2::new(x, y + size / 2.0),
                    Vec2::new(x - size / 3.0, y - size / 4.0),
                    Vec2::new(x + size / 3.0, y - size / 4.0),
                    color,
                );
            },
        }
        
        // Draw outline for better visibility
        let outline_color = Color::new(0.0, 0.0, 0.0, 0.8);
        match self.appearance.shape {
            EntityShape::Circle => {
                draw_circle_lines(
                    self.position.x,
                    self.position.y,
                    self.appearance.size,
                    2.0,
                    outline_color,
                );
            },
            _ => {
                // Simple outline for other shapes
                draw_circle_lines(
                    self.position.x,
                    self.position.y,
                    self.appearance.size + 2.0,
                    1.0,
                    outline_color,
                );
            },
        }
        
        // Show stats if enabled
        if self.appearance.show_stats {
            let stats_text = format!(
                "ID: {}\nGiven: {:.1}\nReceived: {:.1}\nIntensity: {:.2}",
                self.id,
                self.total_emotions_given,
                self.total_emotions_received,
                self.emotions.total_intensity()
            );
            
            draw_text(
                &stats_text,
                self.position.x + self.appearance.size + 5.0,
                self.position.y - 20.0,
                12.0,
                BLACK,
            );
        }
        
        // Show individual emotions as small colored dots
        if self.appearance.show_emotions && !self.emotions.is_empty() {
            let mut offset = 0.0;
            for emotion in self.emotions.emotions.values() {
                if emotion.intensity > 0.1 {
                    draw_circle(
                        self.position.x - self.appearance.size - 5.0,
                        self.position.y - self.appearance.size + offset,
                        3.0,
                        emotion.visual_color(),
                    );
                    offset += 8.0;
                }
            }
        }
    }
    
    /// Get distance to another entity
    pub fn distance_to(&self, other: &Entity) -> f32 {
        (self.position - other.position).length()
    }
    
    /// Check if this entity can interact with another (within range)
    pub fn can_interact_with(&self, other: &Entity, max_distance: f32) -> bool {
        self.active && other.active && self.distance_to(other) <= max_distance
    }
} 