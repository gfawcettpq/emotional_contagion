use macroquad::prelude::*;

/// Simple emotion types for our Conway's Game of Life layers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmotionType {
    Joy,
    Sadness,
    Anger,
    Fear,
    Disgust,
    Anxiety,
    Love,
    Envy,
    Embarrassment,
}

impl EmotionType {
    pub fn all() -> Vec<EmotionType> {
        vec![
            EmotionType::Joy,
            EmotionType::Sadness,
            EmotionType::Anger,
            EmotionType::Fear,
            EmotionType::Disgust,
            EmotionType::Anxiety,
            EmotionType::Love,
            EmotionType::Envy,
            EmotionType::Embarrassment,
        ]
    }

    pub fn color(&self) -> Color {
        match self {
            EmotionType::Joy => Color::new(0.64, 0.75, 0.55, 1.0),        // #a3be8c - Green
            EmotionType::Sadness => Color::new(0.37, 0.51, 0.68, 1.0),    // #5e81ac - Blue
            EmotionType::Anger => Color::new(0.75, 0.38, 0.42, 1.0),      // #bf616a - Red
            EmotionType::Fear => Color::new(0.71, 0.56, 0.68, 1.0),       // #b48ead - Purple
            EmotionType::Disgust => Color::new(0.92, 0.80, 0.55, 1.0),    // #ebcb8b - Yellow
            EmotionType::Anxiety => Color::new(0.64, 0.75, 0.55, 1.0),    // #a3be8c - Light green
            EmotionType::Love => Color::new(0.85, 0.87, 0.91, 1.0),       // #d8dee9 - White
            EmotionType::Envy => Color::new(0.37, 0.51, 0.68, 1.0),       // #5e81ac - Dark blue
            EmotionType::Embarrassment => Color::new(0.71, 0.56, 0.68, 1.0), // #b48ead - Purple
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            EmotionType::Joy => "Joy",
            EmotionType::Sadness => "Sadness",
            EmotionType::Anger => "Anger",
            EmotionType::Fear => "Fear",
            EmotionType::Disgust => "Disgust",
            EmotionType::Anxiety => "Anxiety",
            EmotionType::Love => "Love",
            EmotionType::Envy => "Envy",
            EmotionType::Embarrassment => "Embarrassment",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            EmotionType::Joy => "😊",
            EmotionType::Sadness => "😢",
            EmotionType::Anger => "😡",
            EmotionType::Fear => "😨",
            EmotionType::Disgust => "🤢",
            EmotionType::Anxiety => "😰",
            EmotionType::Love => "💕",
            EmotionType::Envy => "😒",
            EmotionType::Embarrassment => "😳",
        }
    }
}

/// A single cell in our emotion grid - matches the Python version
#[derive(Debug, Clone)]
pub struct EmotionCell {
    pub alive: bool,
    pub emotion: Option<EmotionType>,
    pub intensity: f32, // 0.0 to 1.0
}

impl EmotionCell {
    pub fn new() -> Self {
        Self {
            alive: false,
            emotion: None,
            intensity: 0.0,
        }
    }

    pub fn new_with_emotion(emotion: EmotionType, intensity: f32) -> Self {
        Self {
            alive: true,
            emotion: Some(emotion),
            intensity: intensity.max(0.0).min(1.0),
        }
    }

    pub fn display_color(&self) -> Color {
        if self.alive && self.emotion.is_some() && self.intensity > 0.1 {
            let emotion = self.emotion.unwrap();
            let mut color = emotion.color();
            
            // Adjust brightness based on intensity (0.3 to 1.0 range)
            let brightness = 0.3 + (self.intensity * 0.7);
            color.r *= brightness;
            color.g *= brightness;
            color.b *= brightness;
            
            color
        } else if self.alive {
            // Alive but no emotion - gray
            Color::new(0.4, 0.4, 0.4, 1.0)
        } else {
            // Dead cell - transparent
            Color::new(0.0, 0.0, 0.0, 0.0)
        }
    }
}

/// Stats for tracking what's happening
#[derive(Debug, Clone)]
pub struct GridStats {
    pub alive_count: u32,
    pub emotion_count: u32,
    pub births: u32,
    pub deaths: u32,
    pub emotion_spreads: u32,
}

impl GridStats {
    pub fn new() -> Self {
        Self {
            alive_count: 0,
            emotion_count: 0,
            births: 0,
            deaths: 0,
            emotion_spreads: 0,
        }
    }
}