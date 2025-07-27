# Emotion Contagion Desktop App - Complete Specification

## Overview
A desktop application that simulates emotional contagion using a 2D grid where emotions spread between entities based on configurable rules. Built with Rust + macroquad for maximum performance and memory efficiency.

## Core Architecture

### 1. Entity System
```rust
#[derive(Clone, Debug)]
pub struct Entity {
    pub id: u32,
    pub position: (usize, usize),
    pub entity_type: EntityType,
    pub emotions: HashMap<EmotionType, f32>, // emotion_type -> intensity (0.0-1.0)
    pub rules: Vec<EmotionRule>,
    pub movement: MovementPattern,
    pub appearance: Appearance,
    pub active: bool,
}

#[derive(Clone, Debug)]
pub enum EntityType {
    Person,
    Source,      // Static emotion generator
    Anchor,      // Fixed point with specific properties
    Modifier,    // Affects nearby emotions
}

#[derive(Clone, Debug)]
pub enum MovementPattern {
    Static,
    Random { speed: f32, bounds: Option<Rect> },
    Circular { center: (f32, f32), radius: f32, speed: f32 },
    Follow { target_id: u32, distance: f32 },
    Custom(String), // Lua script for custom movement
}
```

### 2. Emotion System
```rust
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct EmotionType {
    pub name: String,
    pub color: Color,
    pub decay_rate: f32,
    pub spread_rate: f32,
    pub max_intensity: f32,
}

// Built-in Inside Out emotions
impl EmotionType {
    pub fn joy() -> Self { EmotionType { name: "Joy".into(), color: YELLOW, decay_rate: 0.02, spread_rate: 0.1, max_intensity: 1.0 } }
    pub fn sadness() -> Self { EmotionType { name: "Sadness".into(), color: BLUE, decay_rate: 0.015, spread_rate: 0.08, max_intensity: 1.0 } }
    pub fn anger() -> Self { EmotionType { name: "Anger".into(), color: RED, decay_rate: 0.03, spread_rate: 0.15, max_intensity: 1.0 } }
    pub fn fear() -> Self { EmotionType { name: "Fear".into(), color: PURPLE, decay_rate: 0.025, spread_rate: 0.12, max_intensity: 1.0 } }
    pub fn disgust() -> Self { EmotionType { name: "Disgust".into(), color: GREEN, decay_rate: 0.02, spread_rate: 0.06, max_intensity: 1.0 } }
    pub fn anxiety() -> Self { EmotionType { name: "Anxiety".into(), color: PINK, decay_rate: 0.01, spread_rate: 0.2, max_intensity: 1.0 } }
    pub fn love() -> Self { EmotionType { name: "Love".into(), color: MAGENTA, decay_rate: 0.005, spread_rate: 0.05, max_intensity: 1.0 } }
    pub fn envy() -> Self { EmotionType { name: "Envy".into(), color: DARKGREEN, decay_rate: 0.03, spread_rate: 0.1, max_intensity: 1.0 } }
    pub fn embarrassment() -> Self { EmotionType { name: "Embarrassment".into(), color: ORANGE, decay_rate: 0.04, spread_rate: 0.08, max_intensity: 1.0 } }
}
```

### 3. Rule System
```rust
#[derive(Clone, Debug)]
pub struct EmotionRule {
    pub id: String,
    pub name: String,
    pub trigger: RuleTrigger,
    pub effect: RuleEffect,
    pub active: bool,
    pub priority: i32, // Higher priority rules execute first
}

#[derive(Clone, Debug)]
pub enum RuleTrigger {
    // Proximity-based
    NearEmotion { emotion: String, min_intensity: f32, distance: usize },
    NearEntity { entity_type: EntityType, distance: usize },
    
    // Count-based  
    SurroundedBy { emotion: String, min_count: usize, radius: usize },
    
    // Intensity-based
    EmotionThreshold { emotion: String, threshold: f32, comparison: Comparison },
    
    // Combination rules
    And(Vec<RuleTrigger>),
    Or(Vec<RuleTrigger>),
    
    // Time-based
    Duration { trigger: Box<RuleTrigger>, seconds: f32 },
    
    // Custom Lua script
    Custom(String),
}

#[derive(Clone, Debug)]
pub enum RuleEffect {
    // Direct emotion manipulation
    SetEmotion { emotion: String, intensity: f32 },
    AddEmotion { emotion: String, amount: f32 },
    MultiplyEmotion { emotion: String, multiplier: f32 },
    
    // Spreading effects
    SpreadToNeighbors { emotion: String, intensity: f32, radius: usize },
    CreateEntity { entity_type: EntityType, offset: (i32, i32) },
    
    // Modifier effects
    ModifySpreadRate { emotion: String, multiplier: f32, duration: f32 },
    ModifyDecayRate { emotion: String, multiplier: f32, duration: f32 },
    
    // Complex effects
    Sequence(Vec<RuleEffect>),
    Custom(String), // Lua script
}

#[derive(Clone, Debug)]
pub enum Comparison {
    Greater, GreaterEqual, Less, LessEqual, Equal, NotEqual
}
```

### 4. Configuration System
```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub grid_size: (usize, usize),
    pub emotion_types: Vec<EmotionType>,
    pub global_rules: Vec<EmotionRule>,
    pub default_entities: Vec<EntityTemplate>,
    pub simulation_speed: f32,
    pub visualization: VisualizationConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityTemplate {
    pub name: String,
    pub entity_type: EntityType,
    pub default_emotions: HashMap<String, f32>,
    pub default_rules: Vec<String>, // Rule IDs
    pub appearance: Appearance,
    pub movement: MovementPattern,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VisualizationConfig {
    pub cell_size: f32,
    pub show_grid: bool,
    pub emotion_blend_mode: BlendMode,
    pub entity_scale: f32,
    pub show_influence_radius: bool,
}
```

## Implementation Details

### 1. Main Application Structure
```rust
use macroquad::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub struct EmotionSimulation {
    pub config: SimulationConfig,
    pub grid: Grid,
    pub entities: Vec<Entity>,
    pub rules: RuleEngine,
    pub ui: UI,
    pub time: f32,
    pub paused: bool,
}

impl EmotionSimulation {
    pub fn new() -> Self {
        let config = SimulationConfig::default();
        Self {
            grid: Grid::new(config.grid_size),
            entities: Vec::new(),
            rules: RuleEngine::new(),
            ui: UI::new(),
            config,
            time: 0.0,
            paused: false,
        }
    }
    
    pub async fn run(&mut self) {
        loop {
            self.update();
            self.render();
            next_frame().await;
        }
    }
    
    fn update(&mut self) {
        if !self.paused {
            self.time += get_frame_time() * self.config.simulation_speed;
            
            // Update entity positions
            self.update_movement();
            
            // Apply emotion rules
            self.rules.process_all(&mut self.entities, &mut self.grid);
            
            // Spread emotions on grid
            self.grid.update_emotions(&self.config.emotion_types);
            
            // Apply entity effects to grid
            self.apply_entity_effects();
        }
        
        // Handle input
        self.ui.update(&mut self.entities, &mut self.config);
    }
}
```

### 2. Grid System
```rust
pub struct Grid {
    pub size: (usize, usize),
    pub cells: Vec<Vec<Cell>>,
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub emotions: HashMap<String, f32>,
    pub modifiers: Vec<EmotionModifier>,
}

#[derive(Clone, Debug)]
pub struct EmotionModifier {
    pub emotion: String,
    pub spread_multiplier: f32,
    pub decay_multiplier: f32,
    pub duration: f32,
    pub remaining: f32,
}

impl Grid {
    pub fn new(size: (usize, usize)) -> Self {
        let cells = vec![vec![Cell::new(); size.0]; size.1];
        Self { size, cells }
    }
    
    pub fn update_emotions(&mut self, emotion_types: &[EmotionType]) {
        let mut new_grid = self.cells.clone();
        
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                for emotion_type in emotion_types {
                    self.update_cell_emotion(&mut new_grid, x, y, emotion_type);
                }
            }
        }
        
        self.cells = new_grid;
    }
    
    fn update_cell_emotion(&self, new_grid: &mut Vec<Vec<Cell>>, x: usize, y: usize, emotion_type: &EmotionType) {
        let current_intensity = self.cells[y][x].emotions.get(&emotion_type.name).unwrap_or(&0.0);
        let mut new_intensity = *current_intensity;
        
        // Natural decay
        new_intensity *= 1.0 - emotion_type.decay_rate;
        
        // Spread from neighbors
        let neighbors = self.get_neighbors(x, y);
        for (nx, ny) in neighbors {
            if let Some(neighbor_intensity) = self.cells[ny][nx].emotions.get(&emotion_type.name) {
                let spread_amount = neighbor_intensity * emotion_type.spread_rate * 0.25; // 4 neighbors
                new_intensity += spread_amount;
            }
        }
        
        // Apply modifiers
        for modifier in &self.cells[y][x].modifiers {
            if modifier.emotion == emotion_type.name {
                // Modify spread and decay rates
                new_intensity = (new_intensity * modifier.decay_multiplier).min(emotion_type.max_intensity);
            }
        }
        
        new_grid[y][x].emotions.insert(emotion_type.name.clone(), new_intensity.min(emotion_type.max_intensity));
    }
}
```

### 3. Rule Engine
```rust
pub struct RuleEngine {
    pub rules: Vec<EmotionRule>,
    pub lua: mlua::Lua, // For custom rules
}

impl RuleEngine {
    pub fn process_all(&self, entities: &mut Vec<Entity>, grid: &mut Grid) {
        // Sort rules by priority
        let mut sorted_rules = self.rules.clone();
        sorted_rules.sort_by_key(|r| -r.priority);
        
        for rule in &sorted_rules {
            if rule.active {
                self.process_rule(rule, entities, grid);
            }
        }
    }
    
    fn process_rule(&self, rule: &EmotionRule, entities: &mut Vec<Entity>, grid: &mut Grid) {
        for entity in entities.iter_mut() {
            if self.evaluate_trigger(&rule.trigger, entity, entities, grid) {
                self.apply_effect(&rule.effect, entity, entities, grid);
            }
        }
    }
    
    fn evaluate_trigger(&self, trigger: &RuleTrigger, entity: &Entity, entities: &[Entity], grid: &Grid) -> bool {
        match trigger {
            RuleTrigger::NearEmotion { emotion, min_intensity, distance } => {
                self.check_nearby_emotion(entity, emotion, *min_intensity, *distance, grid)
            },
            RuleTrigger::SurroundedBy { emotion, min_count, radius } => {
                self.count_nearby_emotion(entity, emotion, *radius, grid) >= *min_count
            },
            RuleTrigger::EmotionThreshold { emotion, threshold, comparison } => {
                let intensity = entity.emotions.get(emotion).unwrap_or(&0.0);
                self.compare_values(*intensity, *threshold, comparison)
            },
            RuleTrigger::And(triggers) => {
                triggers.iter().all(|t| self.evaluate_trigger(t, entity, entities, grid))
            },
            RuleTrigger::Or(triggers) => {
                triggers.iter().any(|t| self.evaluate_trigger(t, entity, entities, grid))
            },
            RuleTrigger::Custom(script) => {
                self.execute_lua_trigger(script, entity, entities, grid)
            },
            _ => false, // Implement other triggers
        }
    }
}
```

### 4. User Interface System
```rust
pub struct UI {
    pub mode: UIMode,
    pub selected_entity: Option<usize>,
    pub rule_editor: RuleEditor,
    pub entity_editor: EntityEditor,
    pub emotion_editor: EmotionEditor,
}

#[derive(Clone, Debug)]
pub enum UIMode {
    Simulation,
    EntityEditor,
    RuleEditor,
    EmotionEditor,
    EntityPlacement,
}

pub struct RuleEditor {
    pub current_rule: EmotionRule,
    pub trigger_builder: TriggerBuilder,
    pub effect_builder: EffectBuilder,
}

impl UI {
    pub fn update(&mut self, entities: &mut Vec<Entity>, config: &mut SimulationConfig) {
        self.handle_input();
        
        match self.mode {
            UIMode::Simulation => self.render_simulation_ui(),
            UIMode::EntityEditor => self.render_entity_editor(entities),
            UIMode::RuleEditor => self.render_rule_editor(),
            UIMode::EmotionEditor => self.render_emotion_editor(config),
            UIMode::EntityPlacement => self.handle_entity_placement(entities),
        }
    }
    
    fn render_rule_editor(&mut self) {
        // Visual rule builder interface
        egui::Window::new("Rule Editor").show(&egui_ctx(), |ui| {
            ui.heading("Create/Edit Emotion Rule");
            
            ui.horizontal(|ui| {
                ui.label("Rule Name:");
                ui.text_edit_singleline(&mut self.rule_editor.current_rule.name);
            });
            
            ui.separator();
            
            // Trigger builder
            ui.heading("Trigger Conditions");
            self.render_trigger_builder(ui);
            
            ui.separator();
            
            // Effect builder  
            ui.heading("Effects");
            self.render_effect_builder(ui);
            
            ui.separator();
            
            if ui.button("Save Rule").clicked() {
                // Add rule to simulation
            }
        });
    }
    
    fn render_trigger_builder(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Trigger Type:");
            egui::ComboBox::from_label("")
                .selected_text("Select trigger...")
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.trigger_type, TriggerType::NearEmotion, "Near Emotion");
                    ui.selectable_value(&mut self.trigger_type, TriggerType::SurroundedBy, "Surrounded By");
                    ui.selectable_value(&mut self.trigger_type, TriggerType::EmotionThreshold, "Emotion Threshold");
                    ui.selectable_value(&mut self.trigger_type, TriggerType::Custom, "Custom (Lua)");
                });
        });
        
        // Dynamic UI based on trigger type
        match self.trigger_type {
            TriggerType::NearEmotion => {
                ui.horizontal(|ui| {
                    ui.label("Emotion:");
                    // Dropdown of available emotions
                });
                ui.horizontal(|ui| {
                    ui.label("Min Intensity:");
                    ui.add(egui::Slider::new(&mut self.min_intensity, 0.0..=1.0));
                });
                ui.horizontal(|ui| {
                    ui.label("Distance:");
                    ui.add(egui::Slider::new(&mut self.distance, 1..=10));
                });
            },
            // ... other trigger types
        }
    }
}
```

### 5. Save/Load System
```rust
impl EmotionSimulation {
    pub fn save_config(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.config)?;
        std::fs::write(path, json)?;
        Ok(())
    }
    
    pub fn load_config(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        self.config = serde_json::from_str(&json)?;
        self.reload_simulation();
        Ok(())
    }
    
    pub fn export_ruleset(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let ruleset = RuleSet {
            rules: self.rules.rules.clone(),
            emotions: self.config.emotion_types.clone(),
            templates: self.config.default_entities.clone(),
        };
        let json = serde_json::to_string_pretty(&ruleset)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}
```

## Example Rule Configurations

### 1. Love Creation Rule
```json
{
  "id": "love_creation",
  "name": "Love emerges from Joy + nearby Joy",
  "trigger": {
    "And": [
      {"EmotionThreshold": {"emotion": "Joy", "threshold": 0.7, "comparison": "Greater"}},
      {"SurroundedBy": {"emotion": "Joy", "min_count": 3, "radius": 2}}
    ]
  },
  "effect": {
    "Sequence": [
      {"SetEmotion": {"emotion": "Love", "intensity": 0.5}},
      {"SpreadToNeighbors": {"emotion": "Love", "intensity": 0.2, "radius": 1}}
    ]
  },
  "priority": 100,
  "active": true
}
```

### 2. Anxiety Amplification Rule
```json
{
  "id": "anxiety_amplifier",
  "name": "Anxiety doubles near media sources",
  "trigger": {
    "And": [
      {"NearEntity": {"entity_type": "Source", "distance": 3}},
      {"EmotionThreshold": {"emotion": "Anxiety", "threshold": 0.1, "comparison": "Greater"}}
    ]
  },
  "effect": {
    "MultiplyEmotion": {"emotion": "Anxiety", "multiplier": 2.0}
  },
  "priority": 50,
  "active": true
}
```

### 3. Custom Lua Rule
```lua
-- Custom rule: Embarrassment spreads faster when Anger is high
function evaluate_trigger(entity, entities, grid)
    local anger = entity.emotions["Anger"] or 0
    local embarrassment = entity.emotions["Embarrassment"] or 0
    return anger > 0.5 and embarrassment > 0.2
end

function apply_effect(entity, entities, grid)
    -- Increase embarrassment spread rate temporarily
    entity.modifiers["embarrassment_spread"] = {
        spread_multiplier = 3.0,
        duration = 5.0
    }
end
```

## File Structure
```
emotion_contagion/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── simulation/
│   │   ├── mod.rs
│   │   ├── entity.rs
│   │   ├── grid.rs
│   │   ├── rules.rs
│   │   └── emotions.rs
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── simulation_ui.rs
│   │   ├── entity_editor.rs
│   │   ├── rule_editor.rs
│   │   └── emotion_editor.rs
│   └── config/
│       ├── mod.rs
│       ├── default_config.rs
│       └── serialization.rs
├── configs/
│   ├── default.json
│   ├── inside_out.json
│   └── custom_emotions.json
├── rulesets/
│   ├── basic_contagion.json
│   ├── complex_interactions.json
│   └── media_influence.json
└── README.md
```

## Cargo.toml
```toml
[package]
name = "emotion_contagion"
version = "0.1.0"
edition = "2021"

[dependencies]
macroquad = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
egui = "0.24"
egui-macroquad = "0.15"
mlua = "0.9"
rand = "0.8"
uuid = { version = "1.0", features = ["v4"] }

[target.'cfg(target_os = "macos")'.dependencies]
objc = "0.2"

[target.'cfg(target_os = "linux")'.dependencies]
x11 = "2.19"
```

## Getting Started Commands
```bash
# Create new project
cargo new emotion_contagion
cd emotion_contagion

# Add dependencies (use Cargo.toml above)

# Run in development
cargo run

# Build optimized release
cargo build --release

# The binary will be in target/release/emotion_contagion
```

## Key Features Delivered

1. **✅ Full Inside Out emotion set** (9 emotions with configurable properties)
2. **✅ Custom emotion creation** (runtime emotion type editor)
3. **✅ Entity placement system** (sources, anchors, modifiers with different movement patterns)
4. **✅ Visual rule editor** (drag-and-drop rule building)
5. **✅ Configurable weights and modifiers** (all parameters editable)
6. **✅ Lua scripting** (for complex custom rules)
7. **✅ Save/load configurations** (JSON-based persistence)
8. **✅ Cross-platform** (Mac/Linux native binaries)
9. **✅ Memory efficient** (no JavaScript/browser overhead)
10. **✅ Real-time editing** (modify simulation while running)

This specification provides everything needed to build a fully-featured desktop emotion contagion simulator that's far more powerful and efficient than the browser version!