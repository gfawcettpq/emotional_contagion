use macroquad::prelude::*;
use crate::simulation::emotions::*;

/// Types of entities in the simulation
#[derive(Clone, Debug)]
pub enum EntityType {
    Person,      // Regular entity that can give and receive emotions
    Source,      // Generates emotions continuously
    Anchor,      // Absorbs emotions (emotion sink)
    Modifier,    // Modifies emotions passing through
}

/// Entity movement patterns
#[derive(Clone, Debug)]
pub enum MovementPattern {
    Static,                                              // Stays in place
    Random { speed: f32, bounds: Option<Rect> },     // Random wandering
    Circular { center: Vec2, radius: f32, speed: f32 }, // Circular motion
    Follow { target_id: u32, distance: f32 },           // Follow another entity
}

/// Shape options for entity rendering
#[derive(Clone, Debug)]
pub enum EntityShape {
    Circle,
    Square,
    Triangle,
    Heart,
    Star,
}

/// Visual appearance configuration
#[derive(Clone, Debug)]
pub struct Appearance {
    pub size: f32,
    pub shape: EntityShape,
    pub base_color: Color,
    pub show_emotions: bool,
    pub show_stats: bool,
}

/// Main entity in the simulation
#[derive(Clone, Debug)]
pub struct Entity {
    pub id: u32,
    pub entity_type: EntityType,
    pub position: Vec2,
    
    // Movement
    pub movement: MovementPattern,
    pub velocity: Vec2,
    pub max_speed: f32,
    
    // Emotions
    pub emotions: EmotionSet,
    pub emotion_capacity: f32,    // Max total emotion intensity
    pub influence_radius: f32,    // How far emotions spread from this entity
    
    // Visual
    pub appearance: Appearance,
    pub is_visible: bool,
    
    // Behavior  
    pub last_update: f64,
    pub energy: f32,             // Affects emotion spread strength
    
    // Stats for display
    pub total_emotions_given: f32,
    pub total_emotions_received: f32,
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
            entity_type,
            position,
            emotions: EmotionSet::new(),
            movement: MovementPattern::Static,
            appearance,
            is_visible: true,
            last_update: macroquad::prelude::get_time(),
            energy: 1.0,
            velocity: Vec2::ZERO,
            max_speed: 100.0,
            emotion_capacity: 100.0,
            influence_radius: 50.0,
            total_emotions_given: 0.0,
            total_emotions_received: 0.0,
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
        let current_time = macroquad::prelude::get_time();
        let time_diff = current_time - self.last_update;
        self.last_update = current_time;

        // Update emotions (decay over time)
        self.emotions.update(time_diff as f32);
        
        // Update movement
        match &self.movement {
            MovementPattern::Static => {},
            
            MovementPattern::Random { speed, bounds: move_bounds } => {
                let bounds = move_bounds.unwrap_or(bounds);
                
                // Change direction occasionally
                if time_diff > 2.0 {
                    self.velocity = Vec2::new(
                        rand::gen_range(-1.0, 1.0) * speed,
                        rand::gen_range(-1.0, 1.0) * speed,
                    );
                    self.last_update = current_time; // Reset timer after direction change
                }
                
                // Move in current direction
                self.position += self.velocity * delta_time;
                
                let new_pos = self.position;
                
                // Bounce off boundaries
                let mut final_pos = new_pos;
                if new_pos.x < bounds.x || new_pos.x > bounds.x + bounds.w {
                    self.velocity.x *= -1.0;
                    final_pos.x = new_pos.x.clamp(bounds.x, bounds.x + bounds.w);
                }
                if new_pos.y < bounds.y || new_pos.y > bounds.y + bounds.h {
                    self.velocity.y *= -1.0;
                    final_pos.y = new_pos.y.clamp(bounds.y, bounds.y + bounds.h);
                }
                
                self.position = final_pos;
            },
            
            MovementPattern::Circular { center, radius, speed } => {
                // Calculate angle based on time for smooth circular motion
                let angle = self.last_update as f32 * speed;
                self.position = *center + Vec2::new(
                    angle.cos() * radius,
                    angle.sin() * radius,
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
        if !self.is_visible {
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
        self.is_visible && other.is_visible && self.distance_to(other) <= max_distance
    }

    /// Create a new person entity
    pub fn new_person(id: u32, position: Vec2) -> Self {
        Self {
            id,
            entity_type: EntityType::Person,
            position,
            emotions: EmotionSet::new(),
            movement: MovementPattern::Random { 
                speed: 50.0, 
                bounds: None 
            },
            appearance: Appearance {
                size: 12.0,
                shape: EntityShape::Circle,
                base_color: WHITE,
                show_emotions: true,
                show_stats: false,
            },
            is_visible: true,
            last_update: macroquad::prelude::get_time(),
            energy: 1.0,
            velocity: Vec2::ZERO,
            max_speed: 100.0,
            emotion_capacity: 100.0,
            influence_radius: 50.0,
            total_emotions_given: 0.0,
            total_emotions_received: 0.0,
        }
    }

    /// Create a new emotion source entity
    pub fn new_source(id: u32, position: Vec2, emotion_type: EmotionType) -> Self {
        let mut emotions = EmotionSet::new();
        let emotion = Emotion::new(emotion_type.clone(), 0.8);
        emotions.add_emotion(emotion);
        
        Self {
            id,
            entity_type: EntityType::Source,
            position,
            emotions,
            movement: MovementPattern::Static,
            appearance: Appearance {
                size: 18.0,
                shape: EntityShape::Square,
                base_color: super::colors::emotion_to_nord_color(&format!("{:?}", emotion_type)),
                show_emotions: true,
                show_stats: true,
            },
            is_visible: true,
            last_update: macroquad::prelude::get_time(),
            energy: 1.0,
            velocity: Vec2::ZERO,
            max_speed: 100.0,
            emotion_capacity: 100.0,
            influence_radius: 50.0,
            total_emotions_given: 0.0,
            total_emotions_received: 0.0,
        }
    }
} 