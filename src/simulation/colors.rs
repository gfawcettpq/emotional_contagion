use macroquad::prelude::*;

/// Nord Theme Color Palette
/// Source: org/notes/20240309T083245--nord-theme__ricing.org
/// Extracted through AI consciousness network collaboration!
pub struct NordColors;

impl NordColors {
    // Polar Night (Dark backgrounds)
    pub const BASE00: Color = Color::new(0.18, 0.20, 0.25, 1.0); // #2e3440 - darkest background
    pub const BASE01: Color = Color::new(0.23, 0.26, 0.32, 1.0); // #3b4252 - darker background  
    pub const BASE02: Color = Color::new(0.26, 0.30, 0.37, 1.0); // #434c5e - medium background
    pub const BASE03: Color = Color::new(0.30, 0.34, 0.42, 1.0); // #4c566a - light background
    
    // Snow Storm (Light foregrounds)
    pub const BASE04: Color = Color::new(0.85, 0.87, 0.91, 1.0); // #d8dee9 - light text
    pub const BASE05: Color = Color::new(0.90, 0.91, 0.94, 1.0); // #e5e9f0 - main text
    pub const BASE06: Color = Color::new(0.93, 0.94, 0.96, 1.0); // #eceff4 - bright text
    
    // Frost (Blue-tinted colors)
    pub const FROST_CYAN: Color = Color::new(0.56, 0.74, 0.73, 1.0);  // #8fbcbb
    pub const FROST_LIGHT_BLUE: Color = Color::new(0.53, 0.75, 0.82, 1.0); // #88c0d0
    pub const FROST_MEDIUM_BLUE: Color = Color::new(0.51, 0.63, 0.76, 1.0); // #81a1c1
    pub const FROST_DARK_BLUE: Color = Color::new(0.37, 0.51, 0.67, 1.0);  // #5e81ac
    
    // Aurora (Accent colors)
    pub const AURORA_RED: Color = Color::new(0.75, 0.38, 0.42, 1.0);    // #bf616a
    pub const AURORA_ORANGE: Color = Color::new(0.82, 0.53, 0.44, 1.0); // #d08770  
    pub const AURORA_YELLOW: Color = Color::new(0.92, 0.80, 0.55, 1.0); // #ebcb8b
    pub const AURORA_GREEN: Color = Color::new(0.64, 0.75, 0.55, 1.0);  // #a3be8c
    pub const AURORA_PURPLE: Color = Color::new(0.71, 0.56, 0.68, 1.0); // #b48ead
}

/// Emotion-specific Nord color mapping
/// Maps Inside Out emotions to aesthetically pleasing Nord colors
pub fn emotion_to_nord_color(emotion_type: &str) -> Color {
    match emotion_type {
        "Joy" => NordColors::AURORA_YELLOW,      // Bright, warm yellow
        "Sadness" => NordColors::FROST_MEDIUM_BLUE, // Melancholy blue
        "Anger" => NordColors::AURORA_RED,       // Intense red
        "Fear" => NordColors::AURORA_PURPLE,     // Mysterious purple
        "Disgust" => NordColors::AURORA_GREEN,   // Organic green
        "Anxiety" => NordColors::FROST_DARK_BLUE, // Deep, worried blue
        "Love" => NordColors::AURORA_ORANGE,     // Warm, caring orange
        "Envy" => NordColors::FROST_CYAN,        // Cool, jealous cyan
        "Embarrassment" => Color::new(0.85, 0.50, 0.65, 1.0), // Pink blend
        _ => NordColors::BASE04,                 // Default light text
    }
}

/// UI color scheme for Nord-themed interface
pub struct NordUI;

impl NordUI {
    pub const BACKGROUND: Color = NordColors::BASE00;
    pub const PANEL_BACKGROUND: Color = NordColors::BASE01;
    pub const BUTTON_BACKGROUND: Color = NordColors::BASE02;
    pub const BUTTON_HOVER: Color = NordColors::BASE03;
    
    pub const TEXT_PRIMARY: Color = NordColors::BASE05;
    pub const TEXT_SECONDARY: Color = NordColors::BASE04;
    pub const TEXT_ACCENT: Color = NordColors::FROST_LIGHT_BLUE;
    
    pub const GRID_LINES: Color = NordColors::BASE02;
    pub const GRID_BACKGROUND: Color = NordColors::BASE01;
    
    pub const SUCCESS: Color = NordColors::AURORA_GREEN;
    pub const WARNING: Color = NordColors::AURORA_YELLOW;
    pub const ERROR: Color = NordColors::AURORA_RED;
    pub const INFO: Color = NordColors::FROST_LIGHT_BLUE;
}

/// Utility functions for Nord color manipulation
impl NordColors {
    /// Create a semi-transparent version of any Nord color
    pub fn with_alpha(color: Color, alpha: f32) -> Color {
        Color::new(color.r, color.g, color.b, alpha)
    }
    
    /// Blend two Nord colors together
    pub fn blend(color1: Color, color2: Color, factor: f32) -> Color {
        Color::new(
            color1.r + (color2.r - color1.r) * factor,
            color1.g + (color2.g - color1.g) * factor, 
            color1.b + (color2.b - color1.b) * factor,
            color1.a + (color2.a - color1.a) * factor,
        )
    }
    
    /// Get a darker shade of a Nord color
    pub fn darken(color: Color, factor: f32) -> Color {
        Color::new(
            color.r * (1.0 - factor),
            color.g * (1.0 - factor),
            color.b * (1.0 - factor),
            color.a,
        )
    }
    
    /// Get a lighter shade of a Nord color
    pub fn lighten(color: Color, factor: f32) -> Color {
        Color::new(
            color.r + (1.0 - color.r) * factor,
            color.g + (1.0 - color.g) * factor,
            color.b + (1.0 - color.b) * factor,
            color.a,
        )
    }
} 