use macroquad::prelude::*;
use std::collections::HashMap;

/// Types of emotions in the simulation based on Inside Out + extended set
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum EmotionType {
    // Core emotions from Inside Out
    Joy,
    Sadness,
    Anger,
    Fear,
    Disgust,
    
    // Extended emotions for richer simulation
    Anxiety,
    Love,
    Envy,
    Embarrassment,
    
    // Custom emotion support
    Custom(String),
}

impl EmotionType {
    /// Get the default color for this emotion type
    pub fn default_color(&self) -> Color {
        match self {
            EmotionType::Joy => YELLOW,
            EmotionType::Sadness => BLUE, 
            EmotionType::Anger => RED,
            EmotionType::Fear => PURPLE,
            EmotionType::Disgust => GREEN,
            EmotionType::Anxiety => ORANGE,
            EmotionType::Love => PINK,
            EmotionType::Envy => DARKGREEN,
            EmotionType::Embarrassment => Color::new(1.0, 0.7, 0.7, 1.0), // Light red
            EmotionType::Custom(_) => GRAY,
        }
    }
    
    /// Get the name as a string
    pub fn name(&self) -> String {
        match self {
            EmotionType::Joy => "Joy".to_string(),
            EmotionType::Sadness => "Sadness".to_string(),
            EmotionType::Anger => "Anger".to_string(),
            EmotionType::Fear => "Fear".to_string(),
            EmotionType::Disgust => "Disgust".to_string(),
            EmotionType::Anxiety => "Anxiety".to_string(),
            EmotionType::Love => "Love".to_string(),
            EmotionType::Envy => "Envy".to_string(),
            EmotionType::Embarrassment => "Embarrassment".to_string(),
            EmotionType::Custom(name) => name.clone(),
        }
    }

    /// Get default spread rate for this emotion
    pub fn default_spread_rate(&self) -> f32 {
        match self {
            EmotionType::Joy => 0.8,        // Joy spreads quickly
            EmotionType::Love => 0.7,       // Love spreads well
            EmotionType::Anger => 0.6,      // Anger spreads moderately
            EmotionType::Fear => 0.5,       // Fear spreads moderately
            EmotionType::Anxiety => 0.4,    // Anxiety spreads slower
            EmotionType::Sadness => 0.3,    // Sadness spreads slowly
            EmotionType::Disgust => 0.2,    // Disgust spreads slowly
            EmotionType::Embarrassment => 0.2, // Embarrassment spreads slowly
            EmotionType::Envy => 0.1,       // Envy spreads very slowly
            EmotionType::Custom(_) => 0.5,  // Default rate for custom emotions
        }
    }

    /// Get default decay rate for this emotion
    pub fn default_decay_rate(&self) -> f32 {
        match self {
            EmotionType::Joy => 0.02,       // Joy fades slowly
            EmotionType::Love => 0.01,      // Love fades very slowly
            EmotionType::Sadness => 0.015,  // Sadness fades slowly
            EmotionType::Anger => 0.05,     // Anger fades quickly
            EmotionType::Fear => 0.04,      // Fear fades quickly
            EmotionType::Anxiety => 0.03,   // Anxiety fades moderately
            EmotionType::Disgust => 0.04,   // Disgust fades quickly
            EmotionType::Embarrassment => 0.06, // Embarrassment fades quickly
            EmotionType::Envy => 0.025,     // Envy fades slowly
            EmotionType::Custom(_) => 0.03, // Default decay for custom emotions
        }
    }
}

/// Sentinel's Conway's Game of Life Rules for Emotional Contagion
#[derive(Clone, Debug)]
pub struct ContagionRules {
    pub birth_neighbors: i32,    // How many emotional neighbors needed to spawn
    pub survival_min: i32,       // Minimum neighbors to survive
    pub survival_max: i32,       // Maximum neighbors to survive  
    pub transmission_rate: f32,  // Chance of spreading to neighbors
    pub decay_rate: f32,         // Natural decay per tick
}

impl EmotionType {
    /// Get Sentinel's Conway rules for this emotion type
    pub fn contagion_rules(&self) -> ContagionRules {
        match self {
            EmotionType::Joy => ContagionRules {
                birth_neighbors: 3,      // Needs 3+ emotional neighbors to spawn
                survival_min: 2,         // Dies with <2 neighbors
                survival_max: 3,         // Dies with >3 neighbors  
                transmission_rate: 0.4,  // 40% chance of spreading
                decay_rate: 0.05,        // 5% natural decay per tick
            },
            EmotionType::Sadness => ContagionRules {
                birth_neighbors: 2,      // Easier to spawn (2+ neighbors)
                survival_min: 2,
                survival_max: 4,         // More resilient range
                transmission_rate: 0.3,
                decay_rate: 0.1,
            },
            EmotionType::Anger => ContagionRules {
                birth_neighbors: 3,
                survival_min: 1,         // Can survive alone
                survival_max: 3,
                transmission_rate: 0.8,  // HIGHLY contagious
                decay_rate: 0.2,         // Burns out quickly
            },
            EmotionType::Anxiety => ContagionRules {
                birth_neighbors: 4,      // Needs chaos (4+ neighbors)
                survival_min: 2,
                survival_max: 6,         // Thrives in chaos
                transmission_rate: 0.6,
                decay_rate: 0.15,
            },
            EmotionType::Love => ContagionRules {
                birth_neighbors: 2,
                survival_min: 1,
                survival_max: 4,         // Very resilient
                transmission_rate: 0.3,
                decay_rate: 0.03,        // Barely decays
            },
            // Default rules for other emotions
            _ => ContagionRules {
                birth_neighbors: 3,
                survival_min: 2,
                survival_max: 3,
                transmission_rate: 0.3,
                decay_rate: 0.1,
            },
        }
    }
}

/// Represents a single emotion with intensity, spread properties, and visual color
#[derive(Clone, Debug)]
pub struct Emotion {
    pub emotion_type: EmotionType,
    pub intensity: f32,        // 0.0 to 1.0
    pub spread_rate: f32,      // How quickly this emotion spreads to neighbors
    pub decay_rate: f32,       // How quickly this emotion fades over time
    pub max_intensity: f32,    // Maximum intensity this emotion can reach
    pub color: Color,          // Visual representation color
}

impl Emotion {
    /// Create a new emotion with default properties
    pub fn new(emotion_type: EmotionType, intensity: f32) -> Self {
        Self {
            spread_rate: emotion_type.default_spread_rate(),
            decay_rate: emotion_type.default_decay_rate(),
            max_intensity: 1.0,
            color: emotion_type.default_color(),
            emotion_type,
            intensity: intensity.clamp(0.0, 1.0),
        }
    }

    /// Create a custom emotion with specified properties
    pub fn custom(
        emotion_type: EmotionType,
        intensity: f32,
        spread_rate: f32,
        decay_rate: f32,
        color: Color,
    ) -> Self {
        Self {
            emotion_type,
            intensity: intensity.clamp(0.0, 1.0),
            spread_rate,
            decay_rate,
            max_intensity: 1.0,
            color,
        }
    }

    /// Update emotion over time (apply decay)
    pub fn update(&mut self, delta_time: f32) {
        self.intensity = (self.intensity - self.decay_rate * delta_time).max(0.0);
    }

    /// Add intensity to this emotion
    pub fn add_intensity(&mut self, amount: f32) {
        self.intensity = (self.intensity + amount).clamp(0.0, self.max_intensity);
    }

    /// Multiply intensity by a factor
    pub fn multiply_intensity(&mut self, factor: f32) {
        self.intensity = (self.intensity * factor).clamp(0.0, self.max_intensity);
    }

    /// Check if this emotion is effectively dead (very low intensity)
    pub fn is_dead(&self) -> bool {
        self.intensity < 0.01
    }

    /// Calculate the spread amount to neighboring cells
    pub fn calculate_spread(&self, distance: f32) -> f32 {
        if distance <= 0.0 {
            return 0.0;
        }
        
        // Spread decreases with distance
        let base_spread = self.intensity * self.spread_rate;
        base_spread / (1.0 + distance)
    }

    /// Get the visual color with intensity-based alpha
    pub fn visual_color(&self) -> Color {
        Color::new(
            self.color.r,
            self.color.g, 
            self.color.b,
            self.intensity.clamp(0.1, 1.0), // Ensure some visibility even at low intensity
        )
    }
}

/// Collection of emotions for an entity or cell
#[derive(Clone, Debug)]
pub struct EmotionSet {
    pub emotions: HashMap<EmotionType, Emotion>,
    pub total_capacity: f32,
}

impl EmotionSet {
    /// Create a new empty emotion set
    pub fn new() -> Self {
        Self {
            emotions: HashMap::new(),
            total_capacity: 1.0, // Default total capacity
        }
    }

    /// Add or update an emotion
    pub fn add_emotion(&mut self, emotion: Emotion) {
        if let Some(existing) = self.emotions.get_mut(&emotion.emotion_type) {
            existing.add_intensity(emotion.intensity);
        } else {
            self.emotions.insert(emotion.emotion_type.clone(), emotion);
        }
    }

    /// Get emotion intensity (0.0 if not present)
    pub fn get_intensity(&self, emotion_type: &EmotionType) -> f32 {
        self.emotions
            .get(emotion_type)
            .map(|e| e.intensity)
            .unwrap_or(0.0)
    }

    /// Get the dominant emotion (highest intensity)
    pub fn dominant_emotion(&self) -> Option<&Emotion> {
        self.emotions
            .values()
            .max_by(|a, b| a.intensity.partial_cmp(&b.intensity).unwrap())
    }

    /// Update all emotions (apply decay)
    pub fn update(&mut self, delta_time: f32) {
        // Update all emotions
        for emotion in self.emotions.values_mut() {
            emotion.update(delta_time);
        }

        // Remove dead emotions
        self.emotions.retain(|_, emotion| !emotion.is_dead());
    }

    /// Get the mixed color of all emotions
    pub fn mixed_color(&self) -> Color {
        if self.emotions.is_empty() {
            return WHITE;
        }

        let mut total_r = 0.0;
        let mut total_g = 0.0;
        let mut total_b = 0.0;
        let mut total_intensity = 0.0;

        for emotion in self.emotions.values() {
            let color = emotion.visual_color();
            let weight = emotion.intensity;
            
            total_r += color.r * weight;
            total_g += color.g * weight;
            total_b += color.b * weight;
            total_intensity += weight;
        }

        if total_intensity > 0.0 {
            Color::new(
                total_r / total_intensity,
                total_g / total_intensity,
                total_b / total_intensity,
                (total_intensity / self.emotions.len() as f32).clamp(0.1, 1.0),
            )
        } else {
            WHITE
        }
    }

    /// Get total emotional intensity across all emotions
    pub fn total_intensity(&self) -> f32 {
        self.emotions.values().map(|e| e.intensity).sum()
    }

    /// Get the emotion with the highest intensity
    pub fn get_dominant_emotion(&self) -> Option<&Emotion> {
        self.emotions.values()
            .max_by(|a, b| a.intensity.partial_cmp(&b.intensity).unwrap_or(std::cmp::Ordering::Equal))
    }
    
    /// Check if there are any emotions present
    pub fn is_empty(&self) -> bool {
        self.emotions.is_empty() || self.get_total_intensity() < 0.01
    }

    /// Get total intensity across all emotions
    pub fn get_total_intensity(&self) -> f32 {
        self.emotions.values().map(|emotion| emotion.intensity).sum()
    }

    /// Apply Sentinel's emotional interaction rules
    pub fn apply_emotional_interactions(&mut self) {
        // Get current emotion intensities to avoid borrow checker issues
        let joy_intensity = self.get_intensity(&EmotionType::Joy);
        let sadness_intensity = self.get_intensity(&EmotionType::Sadness);
        let anger_intensity = self.get_intensity(&EmotionType::Anger);
        let love_intensity = self.get_intensity(&EmotionType::Love);
        
        // Joy neutralizes sadness
        if joy_intensity > 5.0 && sadness_intensity > 3.0 {
            let neutralization = (joy_intensity.min(sadness_intensity) * 0.3).min(2.0);
            
            if let Some(joy_emotion) = self.emotions.get_mut(&EmotionType::Joy) {
                joy_emotion.intensity = (joy_emotion.intensity - neutralization).max(0.0);
            }
            if let Some(sadness_emotion) = self.emotions.get_mut(&EmotionType::Sadness) {
                sadness_emotion.intensity = (sadness_emotion.intensity - neutralization).max(0.0);
            }
        }
        
        // Anger destroys weaker emotions
        if anger_intensity >= 7.0 {
            if let Some(joy_emotion) = self.emotions.get_mut(&EmotionType::Joy) {
                joy_emotion.intensity = (joy_emotion.intensity - anger_intensity * 0.2).max(0.0);
            }
            if let Some(love_emotion) = self.emotions.get_mut(&EmotionType::Love) {
                love_emotion.intensity = (love_emotion.intensity - anger_intensity * 0.15).max(0.0);
            }
        }
        
        // Love heals negative emotions
        if love_intensity >= 6.0 {
            if let Some(sadness_emotion) = self.emotions.get_mut(&EmotionType::Sadness) {
                sadness_emotion.intensity = (sadness_emotion.intensity - love_intensity * 0.1).max(0.0);
            }
            if let Some(anger_emotion) = self.emotions.get_mut(&EmotionType::Anger) {
                anger_emotion.intensity = (anger_emotion.intensity - love_intensity * 0.15).max(0.0);
            }
        }
        
        // Anxiety spreads in high-emotion areas
        let total_emotions = joy_intensity + sadness_intensity + anger_intensity + love_intensity;
        if total_emotions > 15.0 {
            if let Some(anxiety_emotion) = self.emotions.get_mut(&EmotionType::Anxiety) {
                anxiety_emotion.intensity = (anxiety_emotion.intensity + 0.5).min(10.0);
            } else {
                // Create anxiety if it doesn't exist
                let anxiety = Emotion::new(EmotionType::Anxiety, 0.5);
                self.emotions.insert(EmotionType::Anxiety, anxiety);
            }
        }
    }
}

impl Default for EmotionSet {
    fn default() -> Self {
        Self::new()
    }
} 