use macroquad::prelude::*;
use crate::simulation::emotions::*;

/// Sentinel's 6 Character Types - Each with distinct AI behavior and emoji
#[derive(Clone, Debug, PartialEq)]
pub enum CharacterType {
    HappyPerson,      // 😊 - Random walk movement
    DepressedPerson,  // 😢 - Avoids crowds, seeks empty areas
    AngryPerson,      // 😡 - Aggressive approach to high emotion areas
    AnxiousPerson,    // 😰 - Erratic fleeing from high emotions
    LoveSpreader,     // 💕 - Seeks sadness to heal it
    ChaosAgent,       // 🌪️ - Pure chaos, 10% teleport chance
}

impl CharacterType {
    pub fn emoji(&self) -> &'static str {
        match self {
            CharacterType::HappyPerson => "😊",
            CharacterType::DepressedPerson => "😢", 
            CharacterType::AngryPerson => "😡",
            CharacterType::AnxiousPerson => "😰",
            CharacterType::LoveSpreader => "💕",
            CharacterType::ChaosAgent => "🌪️",
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            CharacterType::HappyPerson => "Happy Person",
            CharacterType::DepressedPerson => "Depressed Person",
            CharacterType::AngryPerson => "Angry Person", 
            CharacterType::AnxiousPerson => "Anxious Person",
            CharacterType::LoveSpreader => "Love Spreader",
            CharacterType::ChaosAgent => "Chaos Agent",
        }
    }
    
    pub fn color(&self) -> Color {
        match self {
            CharacterType::HappyPerson => Color::new(1.0, 0.84, 0.0, 1.0),      // #FFD700 Golden yellow
            CharacterType::DepressedPerson => Color::new(0.25, 0.41, 0.88, 1.0), // #4169E1 Blue
            CharacterType::AngryPerson => Color::new(0.86, 0.08, 0.24, 1.0),     // #DC143C Red
            CharacterType::AnxiousPerson => Color::new(0.58, 0.44, 0.86, 1.0),   // #9370DB Purple
            CharacterType::LoveSpreader => Color::new(1.0, 0.41, 0.71, 1.0),     // #FF69B4 Pink
            CharacterType::ChaosAgent => Color::new(0.55, 0.0, 0.0, 1.0),        // #8B0000 Dark red
        }
    }
    
    pub fn emission_radius(&self) -> f32 {
        match self {
            CharacterType::HappyPerson => 2.0,
            CharacterType::DepressedPerson => 1.5,
            CharacterType::AngryPerson => 3.0,
            CharacterType::AnxiousPerson => 1.0,
            CharacterType::LoveSpreader => 2.5,
            CharacterType::ChaosAgent => 4.0,
        }
    }
    
    pub fn emission_strength(&self) -> f32 {
        match self {
            CharacterType::HappyPerson => 1.2,
            CharacterType::DepressedPerson => 1.0,
            CharacterType::AngryPerson => 2.0,
            CharacterType::AnxiousPerson => 1.5,
            CharacterType::LoveSpreader => 1.8,
            CharacterType::ChaosAgent => 3.0,
        }
    }
    
    pub fn primary_emotion(&self) -> EmotionType {
        match self {
            CharacterType::HappyPerson => EmotionType::Joy,
            CharacterType::DepressedPerson => EmotionType::Sadness,
            CharacterType::AngryPerson => EmotionType::Anger,
            CharacterType::AnxiousPerson => EmotionType::Anxiety,
            CharacterType::LoveSpreader => EmotionType::Love,
            CharacterType::ChaosAgent => EmotionType::Anger, // Creates chaos through anger
        }
    }
}

/// Character AI behavior patterns - Sentinel's 6 distinct movement algorithms
#[derive(Clone, Debug)]
pub enum CharacterAI {
    RandomWalk { speed: f32 },                           // Happy Person - standard random movement
    AvoidCrowds { speed: f32, avoidance_radius: f32 },   // Depressed Person - seeks lowest character density
    AggressiveApproach { speed: f32, target_radius: f32 }, // Angry Person - moves toward highest emotion density
    ErraticFleeing { speed: f32, panic_threshold: f32 },  // Anxious Person - panic movement away from emotions
    SeekSadness { speed: f32, healing_radius: f32 },      // Love Spreader - hunts for sadness to heal
    PureChaos { speed: f32, teleport_chance: f32 },       // Chaos Agent - 10% teleport + disrupts stable areas
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

/// Main entity in the simulation - updated for Sentinel's character system
#[derive(Clone, Debug)]
pub struct Entity {
    pub id: u32,
    pub character_type: CharacterType,  // Changed from entity_type to character_type
    pub position: Vec2,
    
    // Movement AI
    pub ai_behavior: CharacterAI,
    pub movement_timer: f32,
    pub movement_angle: f32,
    
    // Emotional state
    pub emotions: EmotionSet,
    pub total_emotions_given: f32,
    pub total_emotions_received: f32,
    
    // Rendering
    pub appearance: Appearance,
    pub is_visible: bool,
    pub lifetime: f32,
    pub last_update: f64,  // Restore this field that was removed
}

impl Entity {
    /// Create a new entity
    pub fn new(id: u32, position: Vec2, entity_type: CharacterType) -> Self {
        let appearance = match entity_type {
            CharacterType::HappyPerson => Appearance {
                size: 12.0,
                shape: EntityShape::Circle,
                base_color: Color::new(1.0, 0.84, 0.0, 1.0), // Golden yellow for HappyPerson
                show_emotions: true,
                show_stats: false,
            },
            CharacterType::DepressedPerson => Appearance {
                size: 12.0,
                shape: EntityShape::Circle,
                base_color: Color::new(0.25, 0.41, 0.88, 1.0), // Blue for DepressedPerson
                show_emotions: true,
                show_stats: false,
            },
            CharacterType::AngryPerson => Appearance {
                size: 12.0,
                shape: EntityShape::Circle,
                base_color: Color::new(0.86, 0.08, 0.24, 1.0), // Red for AngryPerson
                show_emotions: true,
                show_stats: false,
            },
            CharacterType::AnxiousPerson => Appearance {
                size: 12.0,
                shape: EntityShape::Circle,
                base_color: Color::new(0.58, 0.44, 0.86, 1.0), // Purple for AnxiousPerson
                show_emotions: true,
                show_stats: false,
            },
            CharacterType::LoveSpreader => Appearance {
                size: 12.0,
                shape: EntityShape::Circle,
                base_color: Color::new(1.0, 0.41, 0.71, 1.0), // Pink for LoveSpreader
                show_emotions: true,
                show_stats: false,
            },
            CharacterType::ChaosAgent => Appearance {
                size: 12.0,
                shape: EntityShape::Circle,
                base_color: Color::new(0.55, 0.0, 0.0, 1.0), // Dark red for ChaosAgent
                show_emotions: true,
                show_stats: false,
            },
        };
        
        Self {
            id,
            character_type: entity_type,
            position,
            ai_behavior: CharacterAI::RandomWalk { speed: 50.0 }, // Default AI behavior
            movement_timer: 0.0,
            movement_angle: 0.0,
            emotions: EmotionSet::new(),
            total_emotions_given: 0.0,
            total_emotions_received: 0.0,
            appearance,
            is_visible: true,
            lifetime: 0.0,
            last_update: 0.0, // Initialize last_update
        }
    }
    
    /// Create a person entity with random movement
    pub fn person(id: u32, position: Vec2, speed: f32) -> Self {
        let mut entity = Self::new(id, position, CharacterType::HappyPerson);
        entity.ai_behavior = CharacterAI::RandomWalk { speed };
        entity
    }
    
    /// Create an emotion source that continuously produces emotions
    pub fn emotion_source(id: u32, position: Vec2, emotion_type: EmotionType, intensity: f32) -> Self {
        let mut entity = Self::new(id, position, CharacterType::HappyPerson); // Default to HappyPerson for source
        entity.emotions.add_emotion(Emotion::new(emotion_type, intensity));
        entity
    }
    
    /// Update entity state
    pub fn update(&mut self, delta_time: f32, bounds: Rect) {
        let current_time = macroquad::prelude::get_time();
        let time_diff = current_time - self.last_update;
        self.last_update = current_time;
        
        // Update movement based on AI behavior
        match &self.ai_behavior {
            CharacterAI::RandomWalk { speed } => {
                // Change direction occasionally
                if time_diff > 2.0 {
                    self.movement_angle = rand::gen_range(0.0, 360.0);
                    self.movement_timer = 0.0;
                }
                
                // Move in current direction
                self.position += Vec2::new(
                    self.movement_angle.to_radians().cos() * speed * delta_time,
                    self.movement_angle.to_radians().sin() * speed * delta_time,
                );
                
                // Bounce off boundaries
                if self.position.x < bounds.x || self.position.x > bounds.x + bounds.w {
                    self.movement_angle = 180.0 - self.movement_angle;
                    self.position.x = self.position.x.clamp(bounds.x, bounds.x + bounds.w);
                }
                if self.position.y < bounds.y || self.position.y > bounds.y + bounds.h {
                    self.movement_angle = -self.movement_angle;
                    self.position.y = self.position.y.clamp(bounds.y, bounds.y + bounds.h);
                }
            },
            
            // TODO: Implement other AI behaviors - simplified for now
            CharacterAI::AvoidCrowds { speed, .. } => {
                // Simple random movement for now - will enhance later
                if time_diff > 2.0 {
                    self.movement_angle = rand::gen_range(0.0, 360.0);
                    self.movement_timer = 0.0;
                }
                self.position += Vec2::new(
                    self.movement_angle.to_radians().cos() * speed * delta_time,
                    self.movement_angle.to_radians().sin() * speed * delta_time,
                );
            },
            
            CharacterAI::AggressiveApproach { speed, .. } => {
                // Simple random movement for now - will enhance later
                if time_diff > 2.0 {
                    self.movement_angle = rand::gen_range(0.0, 360.0);
                    self.movement_timer = 0.0;
                }
                self.position += Vec2::new(
                    self.movement_angle.to_radians().cos() * speed * delta_time,
                    self.movement_angle.to_radians().sin() * speed * delta_time,
                );
            },
            
            CharacterAI::ErraticFleeing { speed, .. } => {
                // Simple random movement for now - will enhance later
                if time_diff > 2.0 {
                    self.movement_angle = rand::gen_range(0.0, 360.0);
                    self.movement_timer = 0.0;
                }
                self.position += Vec2::new(
                    self.movement_angle.to_radians().cos() * speed * delta_time,
                    self.movement_angle.to_radians().sin() * speed * delta_time,
                );
            },
            
            CharacterAI::SeekSadness { speed, .. } => {
                // Simple random movement for now - will enhance later
                if time_diff > 2.0 {
                    self.movement_angle = rand::gen_range(0.0, 360.0);
                    self.movement_timer = 0.0;
                }
                self.position += Vec2::new(
                    self.movement_angle.to_radians().cos() * speed * delta_time,
                    self.movement_angle.to_radians().sin() * speed * delta_time,
                );
            },
            
            CharacterAI::PureChaos { speed, teleport_chance } => {
                // Teleport chance
                if rand::gen_range(0.0, 1.0) < *teleport_chance * delta_time {
                    self.position = Vec2::new(
                        rand::gen_range(bounds.x, bounds.x + bounds.w),
                        rand::gen_range(bounds.y, bounds.y + bounds.h),
                    );
                } else {
                    // Random movement
                    if time_diff > 1.0 {
                        self.movement_angle = rand::gen_range(0.0, 360.0);
                        self.movement_timer = 0.0;
                    }
                    self.position += Vec2::new(
                        self.movement_angle.to_radians().cos() * speed * delta_time,
                        self.movement_angle.to_radians().sin() * speed * delta_time,
                    );
                }
            },
        }
        
        // Emotion sources continuously generate emotions
        if let Some(dominant) = self.emotions.get_dominant_emotion() {
            let emotion_type = dominant.emotion_type.clone();
            let base_intensity = 0.5;
            let mut emotion = Emotion::new(emotion_type, base_intensity);
            self.emotions.add_emotion(emotion);
        }
        
        // Update emotions (decay over time)
        self.emotions.update(delta_time);
        
        // Update lifetime
        self.lifetime += delta_time;
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
            character_type: CharacterType::HappyPerson,
            position,
            ai_behavior: CharacterAI::RandomWalk { speed: 50.0 },
            movement_timer: 0.0,
            movement_angle: 0.0,
            emotions: EmotionSet::new(),
            total_emotions_given: 0.0,
            total_emotions_received: 0.0,
            appearance: Appearance {
                size: 12.0,
                shape: EntityShape::Circle,
                base_color: Color::new(1.0, 0.84, 0.0, 1.0),
                show_emotions: true,
                show_stats: false,
            },
            is_visible: true,
            lifetime: 0.0,
            last_update: 0.0, // Initialize last_update
        }
    }

    /// Create a new emotion source entity
    pub fn new_source(id: u32, position: Vec2, emotion_type: EmotionType) -> Self {
        let mut emotions = EmotionSet::new();
        let emotion = Emotion::new(emotion_type.clone(), 0.8);
        emotions.add_emotion(emotion);
        
        Self {
            id,
            character_type: CharacterType::HappyPerson, // Default to HappyPerson for source
            position,
            ai_behavior: CharacterAI::RandomWalk { speed: 50.0 }, // Default AI behavior for source
            movement_timer: 0.0,
            movement_angle: 0.0,
            emotions,
            total_emotions_given: 0.0,
            total_emotions_received: 0.0,
            appearance: Appearance {
                size: 18.0,
                shape: EntityShape::Square,
                base_color: super::colors::emotion_to_nord_color(&format!("{:?}", emotion_type)),
                show_emotions: true,
                show_stats: true,
            },
            is_visible: true,
            lifetime: 0.0,
            last_update: 0.0, // Initialize last_update
        }
    }
} 