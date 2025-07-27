use macroquad::prelude::*;
use crate::simulation::emotions::*;

/// Character types that can be dragged and dropped (like the Python version)
#[derive(Debug, Clone)]
pub struct Character {
    pub name: &'static str,
    pub emotion: EmotionType,
    pub color: Color,
    pub emoji: &'static str,
    pub description: &'static str,
}

impl Character {
    pub fn all() -> Vec<Character> {
        vec![
            Character {
                name: "Joy",
                emotion: EmotionType::Joy,
                color: EmotionType::Joy.color(),
                emoji: "😊",
                description: "Spreads happiness and positive energy",
            },
            Character {
                name: "Sadness",
                emotion: EmotionType::Sadness,
                color: EmotionType::Sadness.color(),
                emoji: "😢",
                description: "Creates melancholy and reflection",
            },
            Character {
                name: "Anger",
                emotion: EmotionType::Anger,
                color: EmotionType::Anger.color(),
                emoji: "😡",
                description: "Generates intense, aggressive energy",
            },
            Character {
                name: "Fear",
                emotion: EmotionType::Fear,
                color: EmotionType::Fear.color(),
                emoji: "😨",
                description: "Creates anxiety and uncertainty",
            },
            Character {
                name: "Disgust",
                emotion: EmotionType::Disgust,
                color: EmotionType::Disgust.color(),
                emoji: "🤢",
                description: "Spreads aversion and rejection",
            },
            Character {
                name: "Anxiety",
                emotion: EmotionType::Anxiety,
                color: EmotionType::Anxiety.color(),
                emoji: "😰",
                description: "Creates nervous tension",
            },
            Character {
                name: "Love",
                emotion: EmotionType::Love,
                color: EmotionType::Love.color(),
                emoji: "💕",
                description: "Spreads warmth and connection",
            },
            Character {
                name: "Envy",
                emotion: EmotionType::Envy,
                color: EmotionType::Envy.color(),
                emoji: "😒",
                description: "Creates jealousy and resentment",
            },
            Character {
                name: "Embarrassment",
                emotion: EmotionType::Embarrassment,
                color: EmotionType::Embarrassment.color(),
                emoji: "😳",
                description: "Generates awkwardness",
            },
        ]
    }
}

/// UI for dragging and dropping characters (like the Python toolbox)
pub struct CharacterToolbox {
    pub characters: Vec<Character>,
    pub position: Vec2,
    pub width: f32,
    pub height: f32,
    pub dragging_character: Option<Character>,
    pub drag_offset: Vec2,
}

impl CharacterToolbox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            characters: Character::all(),
            position: Vec2::new(x, y),
            width,
            height,
            dragging_character: None,
            drag_offset: Vec2::ZERO,
        }
    }

    /// Check if a character is being clicked
    pub fn get_character_at_pos(&self, mouse_pos: Vec2) -> Option<Character> {
        if !self.is_inside_toolbox(mouse_pos) {
            return None;
        }

        let rel_x = mouse_pos.x - self.position.x - 20.0;
        let rel_y = mouse_pos.y - self.position.y - 50.0;

        let chars_per_row = 3;
        let col = (rel_x / 100.0) as usize;
        let row = (rel_y / 80.0) as usize;

        let char_index = row * chars_per_row + col;
        if char_index < self.characters.len() {
            Some(self.characters[char_index].clone())
        } else {
            None
        }
    }

    pub fn is_inside_toolbox(&self, mouse_pos: Vec2) -> bool {
        mouse_pos.x >= self.position.x && 
        mouse_pos.x <= self.position.x + self.width &&
        mouse_pos.y >= self.position.y + 50.0 && 
        mouse_pos.y <= self.position.y + self.height
    }

    pub fn start_drag(&mut self, character: Character, mouse_pos: Vec2) {
        self.dragging_character = Some(character);
        
        // Calculate offset from character center
        let rel_x = mouse_pos.x - self.position.x - 20.0;
        let rel_y = mouse_pos.y - self.position.y - 50.0;
        let _chars_per_row = 3;
        let col = (rel_x / 100.0) as usize;
        let row = (rel_y / 80.0) as usize;
        let char_pos_x = self.position.x + 20.0 + col as f32 * 100.0 + 40.0;
        let char_pos_y = self.position.y + 50.0 + row as f32 * 80.0 + 30.0;
        
        self.drag_offset = Vec2::new(mouse_pos.x - char_pos_x, mouse_pos.y - char_pos_y);
    }

    pub fn stop_drag(&mut self) -> Option<Character> {
        let character = self.dragging_character.clone();
        self.dragging_character = None;
        self.drag_offset = Vec2::ZERO;
        character
    }

    /// Render the toolbox (like the Python version)
    pub fn render(&self) {
        // Nord colors
        let panel_bg = Color::new(0.23, 0.26, 0.32, 1.0);  // #3b4252
        let panel_border = Color::new(0.26, 0.30, 0.37, 1.0); // #434c5e
        let accent = Color::new(0.53, 0.75, 0.82, 1.0); // #88c0d0
        let text_primary = Color::new(0.90, 0.91, 0.94, 1.0); // #e5e9f0

        // Toolbox background
        draw_rectangle(self.position.x, self.position.y, self.width, self.height, panel_bg);
        draw_rectangle_lines(self.position.x, self.position.y, self.width, self.height, 2.0, panel_border);

        // Title
        draw_text("🎭 Emotion Characters", self.position.x + 10.0, self.position.y + 30.0, 20.0, accent);

        // Character grid
        let char_x = self.position.x + 20.0;
        let char_y = self.position.y + 50.0;
        let chars_per_row = 3;

        for (i, character) in self.characters.iter().enumerate() {
            let col = i % chars_per_row;
            let row = i / chars_per_row;

            let char_pos_x = char_x + col as f32 * 100.0;
            let char_pos_y = char_y + row as f32 * 80.0;

            // Character box
            draw_rectangle(char_pos_x, char_pos_y, 80.0, 60.0, panel_border);
            draw_rectangle_lines(char_pos_x, char_pos_y, 80.0, 60.0, 2.0, character.color);

            // Character emoji - larger and more visible
            draw_text(character.emoji, char_pos_x + 25.0, char_pos_y + 35.0, 32.0, WHITE);

            // Character name
            draw_text(character.name, char_pos_x + 5.0, char_pos_y + 50.0, 12.0, text_primary);
        }
    }

    /// Render the character being dragged
    pub fn render_dragging(&self, mouse_pos: Vec2) {
        if let Some(ref character) = self.dragging_character {
            let panel_bg = Color::new(0.23, 0.26, 0.32, 1.0);  // #3b4252
            let text_primary = Color::new(0.90, 0.91, 0.94, 1.0); // #e5e9f0

            let drag_x = mouse_pos.x - self.drag_offset.x - 40.0;
            let drag_y = mouse_pos.y - self.drag_offset.y - 30.0;

            // Draw character being dragged
            draw_rectangle(drag_x, drag_y, 80.0, 60.0, panel_bg);
            draw_rectangle_lines(drag_x, drag_y, 80.0, 60.0, 3.0, character.color);

            // Character emoji - larger and more visible
            draw_text(character.emoji, drag_x + 25.0, drag_y + 35.0, 32.0, WHITE);

            // Character name
            draw_text(character.name, drag_x + 5.0, drag_y + 50.0, 12.0, text_primary);
        }
    }
}