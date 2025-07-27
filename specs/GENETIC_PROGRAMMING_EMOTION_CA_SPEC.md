# Genetic Programming Emotion Cellular Automata - Technical Specification

## Overview

A cellular automata system where emotional rules evolve through genetic programming. Combines Conway's Game of Life mechanics with genetic algorithms and multi-dimensional emotion states. **Emotions themselves affect how genetic programs breed and mutate.**

## Core Architecture

### 1. Multi-Dimensional Linked List Grid

```rust
#[derive(Clone, Debug)]
pub struct Cell {
    pub position: (usize, usize),
    pub emotions: [f32; 9], // Joy, Sadness, Anger, Fear, Disgust, Anxiety, Love, Envy, Embarrassment
    pub neighbors: NeighborLinks,
    pub rule_program: GeneticProgram,
    pub fitness_score: f32,
    pub generation: u32,
    pub breeding_history: Vec<BreedingEvent>,
}

#[derive(Clone, Debug)]
pub struct NeighborLinks {
    pub north: Option<CellRef>,
    pub south: Option<CellRef>, 
    pub east: Option<CellRef>,
    pub west: Option<CellRef>,
    pub northeast: Option<CellRef>,
    pub northwest: Option<CellRef>,
    pub southeast: Option<CellRef>,
    pub southwest: Option<CellRef>,
}

impl Cell {
    // Chain neighbor access: cell.north().north().east()
    pub fn north(&self) -> Option<&Cell> { /* ... */ }
    pub fn south(&self) -> Option<&Cell> { /* ... */ }
    pub fn east(&self) -> Option<&Cell> { /* ... */ }
    pub fn west(&self) -> Option<&Cell> { /* ... */ }
    
    // Get multiple neighbors in direction
    pub fn get_neighbors_in_direction(&self, direction: Direction, count: usize) -> Vec<&Cell> { /* ... */ }
    pub fn get_neighbors_in_radius(&self, radius: usize) -> Vec<&Cell> { /* ... */ }
}
```

### 2. Genetic Programming Rule System

```rust
#[derive(Clone, Debug)]
pub struct GeneticProgram {
    pub genes: Vec<Gene>,
    pub combinator: LogicCombinator, // AND, OR, XOR, MAJORITY
    pub mutation_modifiers: EmotionalMutationProfile,
}

#[derive(Clone, Debug)]
pub enum Gene {
    // Neighbor queries
    NeighborCount { direction: Direction, emotion: EmotionType, threshold: f32 },
    NeighborSum { directions: Vec<Direction>, emotion: EmotionType, min_sum: f32 },
    NeighborPattern { pattern: NeighborPattern, emotion: EmotionType },
    
    // Self-state queries  
    SelfEmotion { emotion: EmotionType, threshold: f32, comparison: Comparison },
    EmotionRatio { emotion1: EmotionType, emotion2: EmotionType, min_ratio: f32 },
    
    // Complex patterns
    Surrounded { emotion: EmotionType, min_neighbors: usize },
    Isolated { max_neighbors: usize },
    EdgeOfCluster { emotion: EmotionType, cluster_size: usize },
    
    // Meta-rules
    Composite(Box<GeneticProgram>), // Nested programs
}

#[derive(Clone, Debug)]
pub enum Direction {
    North, South, East, West,
    Northeast, Northwest, Southeast, Southwest,
    Cardinal, // N,S,E,W
    Diagonal, // NE,NW,SE,SW
    All,
}

#[derive(Clone, Debug)]
pub enum NeighborPattern {
    Line { direction: Direction, length: usize },
    Cross { arm_length: usize },
    Square { size: usize },
    Circle { radius: usize },
    Custom(Vec<(i32, i32)>), // Relative positions
}

#[derive(Clone, Debug)]
pub enum LogicCombinator {
    And,    // All genes must be true
    Or,     // Any gene must be true  
    Xor,    // Exactly one gene must be true
    Majority, // More than half genes must be true
    Weighted(Vec<f32>), // Weighted combination with thresholds
}
```

### 3. Emotion-Driven Breeding & Mutation

```rust
#[derive(Clone, Debug)]
pub struct EmotionalMutationProfile {
    pub base_mutation_rate: f32,
    pub emotion_modifiers: [f32; 9], // How each emotion affects mutation
}

impl EmotionalMutationProfile {
    pub fn calculate_mutation_rate(&self, emotions: &[f32; 9]) -> f32 {
        let mut rate = self.base_mutation_rate;
        
        for (i, &emotion_level) in emotions.iter().enumerate() {
            rate += emotion_level * self.emotion_modifiers[i];
        }
        
        rate.clamp(0.0, 1.0)
    }
    
    // Default emotional effects on mutation
    pub fn default() -> Self {
        Self {
            base_mutation_rate: 0.1,
            emotion_modifiers: [
                0.3,  // Joy - increases creativity/exploration
                -0.1, // Sadness - reduces mutation (withdrawal)
                0.2,  // Anger - moderate increase (reckless changes)
                -0.2, // Fear - reduces mutation (conservative)
                -0.05, // Disgust - slight reduction (rejection of change)
                0.15, // Anxiety - slight increase (restless changes)
                0.4,  // Love - highest increase (openness to change)
                0.1,  // Envy - slight increase (desire for different traits)
                -0.15, // Embarrassment - reduction (avoidance behavior)
            ]
        }
    }
}

#[derive(Clone, Debug)]
pub struct BreedingEvent {
    pub partner_position: (usize, usize),
    pub parent_emotions: ([f32; 9], [f32; 9]),
    pub offspring_program: GeneticProgram,
    pub generation: u32,
}

pub struct EmotionalBreeding;

impl EmotionalBreeding {
    pub fn attempt_breeding(
        cell1: &Cell, 
        cell2: &Cell, 
        breeding_threshold: f32
    ) -> Option<GeneticProgram> {
        
        // Check if emotions are compatible for breeding
        let compatibility = Self::calculate_compatibility(&cell1.emotions, &cell2.emotions);
        
        if compatibility < breeding_threshold {
            return None;
        }
        
        // Emotional state affects crossover behavior
        let crossover_style = Self::determine_crossover_style(&cell1.emotions, &cell2.emotions);
        
        // Create offspring program
        let offspring = Self::crossover_programs(
            &cell1.rule_program,
            &cell2.rule_program,
            crossover_style
        );
        
        // Apply emotion-influenced mutation
        let mutation_rate = Self::calculate_combined_mutation_rate(cell1, cell2);
        let mutated_offspring = Self::mutate_program(offspring, mutation_rate, &cell1.emotions);
        
        Some(mutated_offspring)
    }
    
    fn calculate_compatibility(emotions1: &[f32; 9], emotions2: &[f32; 9]) -> f32 {
        // Love and Joy increase compatibility
        // Anger and Fear reduce compatibility
        // Similar emotion levels increase compatibility
        
        let love_factor = (emotions1[6] + emotions2[6]) * 0.3; // Love index
        let joy_factor = (emotions1[0] + emotions2[0]) * 0.2;  // Joy index
        let anger_penalty = (emotions1[2] + emotions2[2]) * -0.2; // Anger index
        let fear_penalty = (emotions1[3] + emotions2[3]) * -0.1; // Fear index
        
        // Similarity bonus
        let similarity = 1.0 - emotions1.iter()
            .zip(emotions2.iter())
            .map(|(a, b)| (a - b).abs())
            .sum::<f32>() / 9.0;
        
        (love_factor + joy_factor + anger_penalty + fear_penalty + similarity * 0.3).clamp(0.0, 1.0)
    }
    
    fn determine_crossover_style(emotions1: &[f32; 9], emotions2: &[f32; 9]) -> CrossoverStyle {
        let avg_love = (emotions1[6] + emotions2[6]) / 2.0;
        let avg_anger = (emotions1[2] + emotions2[2]) / 2.0;
        let avg_anxiety = (emotions1[5] + emotions2[5]) / 2.0;
        
        if avg_love > 0.7 {
            CrossoverStyle::Harmonious // Balanced mixing of genes
        } else if avg_anger > 0.6 {
            CrossoverStyle::Competitive // Dominant genes win
        } else if avg_anxiety > 0.5 {
            CrossoverStyle::Chaotic // Random mixing
        } else {
            CrossoverStyle::Conservative // Preserve successful patterns
        }
    }
}

#[derive(Clone, Debug)]
pub enum CrossoverStyle {
    Harmonious,    // Balanced gene mixing, preserve both parents' strengths
    Competitive,   // Stronger genes dominate, weaker genes discarded
    Chaotic,       // Random mixing with high variability
    Conservative,  // Minimal changes, prefer proven gene combinations
}
```

### 4. Cell State Determination

```rust
pub trait CellStateFunction {
    fn determine_state(&self, emotions: &[f32; 9]) -> CellState;
}

#[derive(Clone, Debug)]
pub enum CellState {
    Dominant(EmotionType), // Highest emotion
    Blended(Vec<(EmotionType, f32)>), // Multiple emotions with weights
    Neutral, // All emotions below threshold
    Custom(String), // Custom state determined by function
}

// Built-in state functions
pub struct HighestEmotionState;
pub struct WeightedBlendState { weights: [f32; 9] };
pub struct ThresholdState { threshold: f32 };
pub struct EmotionalWeightedState; // Emotions affect their own weight in the calculation
```

### 5. Evolution System with Emotional Influence

```rust
#[derive(Clone, Debug)]
pub struct EvolutionEngine {
    pub population: Vec<GeneticProgram>,
    pub fitness_evaluator: Box<dyn FitnessEvaluator>,
    pub emotional_breeding: EmotionalBreeding,
    pub generation_count: u32,
}

impl EvolutionEngine {
    pub fn evolve_generation(&mut self, grid: &mut Grid) {
        // 1. Evaluate fitness considering emotional states
        self.evaluate_population_fitness(grid);
        
        // 2. Attempt breeding between neighboring cells based on emotional compatibility
        let breeding_attempts = self.attempt_emotional_breeding(grid);
        
        // 3. Apply emotion-influenced mutations to existing programs
        self.apply_emotional_mutations(grid);
        
        // 4. Handle cell death and birth based on fitness and emotional state
        self.handle_population_dynamics(grid);
        
        self.generation_count += 1;
    }
    
    fn attempt_emotional_breeding(&self, grid: &mut Grid) -> Vec<BreedingEvent> {
        let mut breeding_events = Vec::new();
        
        for cell in grid.cells() {
            for neighbor in cell.get_neighbors_in_radius(1) {
                if let Some(offspring_program) = EmotionalBreeding::attempt_breeding(
                    cell, 
                    neighbor, 
                    0.6 // breeding threshold
                ) {
                    breeding_events.push(BreedingEvent {
                        partner_position: neighbor.position,
                        parent_emotions: (cell.emotions, neighbor.emotions),
                        offspring_program,
                        generation: self.generation_count,
                    });
                }
            }
        }
        
        breeding_events
    }
    
    fn apply_emotional_mutations(&self, grid: &mut Grid) {
        for cell in grid.cells_mut() {
            let mutation_rate = cell.rule_program.mutation_modifiers
                .calculate_mutation_rate(&cell.emotions);
            
            if rand::random::<f32>() < mutation_rate {
                cell.rule_program = Self::mutate_program_emotionally(
                    &cell.rule_program,
                    &cell.emotions
                );
            }
        }
    }
    
    fn mutate_program_emotionally(
        program: &GeneticProgram, 
        emotions: &[f32; 9]
    ) -> GeneticProgram {
        let mut mutated = program.clone();
        
        // Different emotions cause different types of mutations
        let joy_level = emotions[0];
        let anger_level = emotions[2];
        let love_level = emotions[6];
        
        if joy_level > 0.7 {
            // Joy causes creative mutations - add new genes
            mutated.genes.push(Gene::random_creative());
        }
        
        if anger_level > 0.6 {
            // Anger causes destructive mutations - remove genes
            if mutated.genes.len() > 1 {
                mutated.genes.remove(rand::random::<usize>() % mutated.genes.len());
            }
        }
        
        if love_level > 0.8 {
            // Love causes cooperative mutations - favor neighbor-friendly genes
            mutated.genes.push(Gene::random_cooperative());
        }
        
        mutated
    }
}
```

### 6. Example Emotional Effects

```rust
// Example: How different emotions affect breeding and mutation

// High Love Cell - Very open to breeding, creates harmonious offspring
let loving_cell = Cell {
    emotions: [0.3, 0.1, 0.0, 0.1, 0.0, 0.2, 0.9, 0.0, 0.0], // High love
    rule_program: cooperative_program(),
    // ... other fields
};

// High Anger Cell - Selective breeding, aggressive mutations
let angry_cell = Cell {
    emotions: [0.1, 0.2, 0.8, 0.3, 0.1, 0.4, 0.0, 0.3, 0.1], // High anger
    rule_program: competitive_program(),
    // ... other fields
};

// High Joy Cell - Creative mutations, frequent breeding attempts
let joyful_cell = Cell {
    emotions: [0.9, 0.0, 0.0, 0.0, 0.0, 0.1, 0.3, 0.0, 0.0], // High joy
    rule_program: creative_program(),
    // ... other fields
};

// Fearful Cell - Conservative breeding, minimal mutations
let fearful_cell = Cell {
    emotions: [0.1, 0.3, 0.1, 0.8, 0.2, 0.6, 0.0, 0.0, 0.2], // High fear/anxiety
    rule_program: conservative_program(),
    // ... other fields
};
```

### 7. Simulation Flow

```rust
fn simulation_step(grid: &mut Grid, evolution_engine: &mut EvolutionEngine) {
    // 1. Execute genetic programs for each cell
    for cell in grid.cells_mut() {
        let survives = cell.rule_program.evaluate(cell, &grid);
        if !survives {
            // Emotional death - certain emotions make death more/less likely
            let death_resistance = calculate_emotional_death_resistance(&cell.emotions);
            if rand::random::<f32>() > death_resistance {
                cell.emotions = [0.0; 9]; // Cell dies
            }
        }
    }
    
    // 2. Apply emotion spreading (emotions influence how they spread)
    grid.spread_emotions_with_emotional_influence();
    
    // 3. Attempt emotional breeding between compatible neighbors
    evolution_engine.evolve_generation(grid);
    
    // 4. Update fitness based on survival and emotional harmony
    grid.update_fitness_scores();
}

fn calculate_emotional_death_resistance(emotions: &[f32; 9]) -> f32 {
    let love = emotions[6];
    let joy = emotions[0];
    let fear = emotions[3];
    let sadness = emotions[1];
    
    // Love and joy increase survival, fear and sadness decrease it
    (love * 0.4 + joy * 0.3 - fear * 0.2 - sadness * 0.1).clamp(0.0, 1.0)
}
```

## Advanced Emotional Effects

### 1. Emotional Momentum
- Emotions from previous generations influence current breeding behavior
- Emotional "memory" affects how programs evolve
- Traumatic events (mass cell death) can cause lasting behavioral changes

### 2. Emotional Contagion in Breeding
- Successful emotional patterns spread through breeding networks
- Emotional "epidemics" can sweep through the population
- Resistance and immunity to certain emotional patterns

### 3. Seasonal Emotional Cycles
- Environmental factors change emotional breeding preferences
- Different emotions become advantageous in different "seasons"
- Migration patterns based on emotional compatibility

This creates a truly living emotional ecosystem where the feelings themselves shape evolution!

## String-Encoded Rule Language

### Rule Encoding Syntax

Rules are encoded as human-readable strings that can be easily shared, mutated, and combined. The syntax supports complex neighbor queries and logical operations.

```rust
#[derive(Clone, Debug)]
pub struct RuleString {
    pub rule_text: String,
    pub compiled_rule: CompiledRule,
}

impl RuleString {
    pub fn new(rule_text: &str) -> Result<Self, RuleParseError> {
        let compiled_rule = RuleParser::parse(rule_text)?;
        Ok(Self {
            rule_text: rule_text.to_string(),
            compiled_rule,
        })
    }
}

// Example rule strings:
let conway_rule = RuleString::new("count(all, live) == 3 or (self.live and count(all, live) == 2)")?;
let emotion_rule = RuleString::new("north(4, joy) > 0.5 and south(2, anger) < 0.3")?;
let complex_rule = RuleString::new("pattern(cross, love) and not surrounded(fear, 6) or self.joy > self.sadness * 2")?;
```

### Basic Query Functions

```rust
// Directional queries
"north(N, emotion)"        // N northernmost neighbors with emotion
"south(N, emotion)"        // N southernmost neighbors  
"east(N, emotion)"         // N easternmost neighbors
"west(N, emotion)"         // N westernmost neighbors
"diagonal(N, emotion)"     // N diagonal neighbors
"cardinal(N, emotion)"     // N cardinal (N/S/E/W) neighbors

// Count queries  
"count(direction, emotion)"     // Count neighbors with emotion above threshold
"count_above(direction, emotion, threshold)" // Count above specific threshold
"count_below(direction, emotion, threshold)" // Count below specific threshold
"count_between(direction, emotion, min, max)" // Count in range

// Pattern queries
"pattern(shape, emotion)"   // Check for specific patterns
"line(direction, length, emotion)" // Line of cells with emotion
"cross(size, emotion)"      // Cross pattern
"square(size, emotion)"     // Square pattern  
"circle(radius, emotion)"   // Circle pattern
"cluster(emotion, min_size)" // Cluster of connected cells

// Specific position queries
"cells([rel_positions], emotion)" // Specific relative positions
"neighbors([indices], emotion)"    // Specific neighbor indices

// Self queries
"self.emotion"              // Current cell's emotion level
"self.age"                  // How long cell has been alive
"self.fitness"              // Current fitness score
"self.generation"           // Generation number

// Aggregate functions
"sum(query)"                // Sum values from query
"avg(query)"                // Average values
"max(query)"                // Maximum value
"min(query)"                // Minimum value
"median(query)"             // Median value
```

### Example Rule Strings

```rust
// Conway's Game of Life (classic)
let conway_live = "count(all, live) == 3 or (self.live and count(all, live) == 2)";
let conway_die = "not (count(all, live) == 3 or (self.live and count(all, live) == 2))";

// Emotional Conway variants
let joy_spreads = "count(all, joy) >= 3 and count(all, joy) <= 5";
let love_emerges = "count(all, joy) >= 4 and self.joy > 0.7";
let sadness_isolates = "count(all, sadness) <= 1 or self.sadness > 0.8";

// Complex emotional rules
let social_butterfly = "north(3, joy) > 0.5 and south(3, joy) > 0.5 and east(3, joy) > 0.5 and west(3, joy) > 0.5";
let anxiety_spreader = "pattern(cross, anxiety) and count(all, calm) < 2";
let love_triangle = "cells([(0,1), (1,0), (0,-1)], love) and self.love < 0.3";

// Genetic programming combinable rules
let survival_rule = "sum(cardinal(8, any)) > 2.0 and sum(cardinal(8, any)) < 6.0";
let exploration_rule = "max(diagonal(4, curiosity)) > self.curiosity * 1.5";
let cooperation_rule = "avg(all, happiness) > self.happiness and count(all, anger) == 0";

// Meta-rules that combine other rules
let adaptive_rule = "(generation < 100 and simple_survival) or (generation >= 100 and complex_social)";
let seasonal_rule = "if(time % 1000 < 500, summer_rule, winter_rule)";
```

### Advanced Query Patterns

```rust
// Directional groups
"northernmost(N, emotion)"     // N cells furthest north
"southernmost(N, emotion)"     // N cells furthest south  
"closest(N, emotion)"          // N closest neighbors
"furthest(N, emotion)"         // N furthest neighbors (in radius)

// Pattern matching
"pattern(L_shape, emotion)"    // L-shaped pattern
"pattern(line_NS_3, emotion)"  // North-South line of 3
"pattern(random_5, emotion)"   // Any 5 neighbors

// Relative position queries  
"cells([(0,1), (1,1), (2,1)], emotion)"  // Specific coordinates
"neighbors([0, 2, 4, 6], emotion)"       // Specific neighbor indices (0=N, 1=NE, 2=E, etc.)

// Conditional queries
"if(self.love > 0.5, north(3, joy), south(3, sadness))" // Conditional logic
"when(count(all, anger) > 5, flee_mode, explore_mode)"  // State-dependent rules

// Temporal queries  
"changed(emotion, 3)"          // Emotion changed in last 3 steps
"stable(emotion, 10)"          // Emotion stable for 10 steps
"trending_up(emotion, 5)"      // Emotion increasing over 5 steps
"oscillating(emotion, period)" // Emotion oscillating with period

// Statistical queries
"variance(all, emotion)"       // Variance in neighbor emotions
"correlation(joy, love)"       // Correlation between emotions
"entropy(all)"                 // Emotional entropy of neighborhood
```

### Rule Compilation and Execution

```rust
#[derive(Clone, Debug)]
pub enum CompiledRule {
    Query(QueryOp),
    Comparison(ComparisonOp),
    Logic(LogicOp),
    Conditional(ConditionalOp),
    Function(FunctionOp),
}

#[derive(Clone, Debug)]
pub enum QueryOp {
    Directional { direction: Direction, count: usize, emotion: EmotionType },
    Pattern { pattern: PatternType, emotion: EmotionType },
    Count { area: AreaType, emotion: EmotionType, threshold: f32 },
    Cells { positions: Vec<(i32, i32)>, emotion: EmotionType },
    Self { property: SelfProperty },
}

#[derive(Clone, Debug)]
pub enum PatternType {
    Cross(usize),
    Line(Direction, usize),
    Square(usize),
    Circle(usize),
    LShape,
    Custom(Vec<(i32, i32)>),
}

pub struct RuleParser;

impl RuleParser {
    pub fn parse(rule_text: &str) -> Result<CompiledRule, RuleParseError> {
        // Parse rule string into compiled representation
        // Support for:
        // - Function calls: north(4, joy)
        // - Comparisons: > < == != >= <=
        // - Logic: and, or, not, xor
        // - Conditionals: if(condition, true_rule, false_rule)
        // - Grouping: (expression)
        // - Self references: self.emotion
        
        let tokens = Self::tokenize(rule_text)?;
        let ast = Self::parse_tokens(tokens)?;
        let compiled = Self::compile_ast(ast)?;
        
        Ok(compiled)
    }
    
    pub fn evaluate(rule: &CompiledRule, cell: &Cell, grid: &Grid) -> f32 {
        // Execute compiled rule and return result (0.0-1.0)
        match rule {
            CompiledRule::Query(query_op) => Self::evaluate_query(query_op, cell, grid),
            CompiledRule::Comparison(comp_op) => Self::evaluate_comparison(comp_op, cell, grid),
            CompiledRule::Logic(logic_op) => Self::evaluate_logic(logic_op, cell, grid),
            CompiledRule::Conditional(cond_op) => Self::evaluate_conditional(cond_op, cell, grid),
            CompiledRule::Function(func_op) => Self::evaluate_function(func_op, cell, grid),
        }
    }
}

// Example rule evaluation
let rule = RuleString::new("north(4, joy) > 0.5 and count(all, anger) < 3")?;
let survives = RuleParser::evaluate(&rule.compiled_rule, &cell, &grid) > 0.5;
```

### Genetic Programming with String Rules

```rust
#[derive(Clone, Debug)]
pub struct StringBasedGeneticProgram {
    pub rule_strings: Vec<String>,
    pub combinator: String, // "and", "or", "majority", "weighted(0.3,0.7)"
}

impl StringBasedGeneticProgram {
    pub fn mutate(&mut self) {
        // Mutate individual rule components
        for rule in &mut self.rule_strings {
            if rand::random::<f32>() < 0.1 {
                *rule = Self::mutate_rule_string(rule);
            }
        }
        
        // Mutate combinator
        if rand::random::<f32>() < 0.05 {
            self.combinator = Self::random_combinator();
        }
    }
    
    fn mutate_rule_string(rule: &str) -> String {
        // Examples of mutations:
        // "north(4, joy)" -> "north(3, joy)" (change parameter)
        // "north(4, joy)" -> "south(4, joy)" (change direction)  
        // "north(4, joy)" -> "north(4, love)" (change emotion)
        // "count(all, anger) < 3" -> "count(all, anger) < 4" (change threshold)
        // Add random function: "... and diagonal(2, fear) > 0.2"
        
        // Implementation would parse and randomly modify components
        todo!("Implement rule string mutation")
    }
    
    pub fn crossover(parent1: &Self, parent2: &Self) -> Self {
        // Mix rule strings from both parents
        let mut offspring_rules = Vec::new();
        
        // Take some rules from each parent
        offspring_rules.extend(parent1.rule_strings.iter().take(2).cloned());
        offspring_rules.extend(parent2.rule_strings.iter().take(2).cloned());
        
        // Combine combinators
        let combinator = if rand::random::<bool>() {
            parent1.combinator.clone()
        } else {
            parent2.combinator.clone()
        };
        
        Self {
            rule_strings: offspring_rules,
            combinator,
        }
    }
}

// Example genetic program evolution
let parent1 = StringBasedGeneticProgram {
    rule_strings: vec![
        "north(3, joy) > 0.4".to_string(),
        "count(all, anger) < 2".to_string(),
    ],
    combinator: "and".to_string(),
};

let parent2 = StringBasedGeneticProgram {
    rule_strings: vec![
        "south(4, love) > 0.6".to_string(),
        "self.happiness > self.sadness".to_string(),
    ],
    combinator: "or".to_string(),
};

let offspring = StringBasedGeneticProgram::crossover(&parent1, &parent2);
// Result might be:
// rule_strings: ["north(3, joy) > 0.4", "count(all, anger) < 2", "south(4, love) > 0.6", "self.happiness > self.sadness"]
// combinator: "and" (or "or" randomly chosen)
```

### Backwards Compatibility Examples

```rust
// Classic Conway's Game of Life rules expressed in the new syntax
let conway_survive = RuleString::new("count(all, live) == 2 or count(all, live) == 3")?;
let conway_born = RuleString::new("not self.live and count(all, live) == 3")?;

// Easy migration to emotional rules
let emotional_conway = RuleString::new("count(all, joy) == 2 or count(all, joy) == 3")?;

// Progressive complexity
let simple_rule = RuleString::new("north(1, happiness) > 0.5")?;
let medium_rule = RuleString::new("north(3, joy) > 0.4 and south(2, anger) < 0.3")?;
let complex_rule = RuleString::new(
    "pattern(cross, love) and not surrounded(fear, 6) and if(self.age > 10, stable(joy, 5), true)"
)?;

// Chainable and composable
let base_survival = "count(all, any) >= 2 and count(all, any) <= 5";
let social_bonus = "avg(cardinal(4, happiness)) > self.happiness";
let combined_rule = format!("({}) and ({})", base_survival, social_bonus);
```

This string-based rule system makes it easy to:
1. Share rules between cells and experiments
2. Mutate rules through string manipulation  
3. Create rule libraries and templates
4. Debug and visualize rule behavior
5. Start simple (Conway) and scale up to complex emotional patterns
6. Mix and match rule components genetically

The rules become the DNA that can evolve, breed, and adapt!

## Interactive Visualization & Debugging System

### Multi-Scale Visualization

```rust
#[derive(Clone, Debug)]
pub enum ViewMode {
    System,      // Full grid overview with emotional heat maps
    Neighborhood(Position, u32), // Local area around position with radius
    Cell(Position), // Individual cell with full detail
    Rule(String),   // Rule execution visualization
    Lineage(CellId), // Genetic lineage tree
}

#[derive(Clone, Debug)]
pub struct VisualizationState {
    pub current_mode: ViewMode,
    pub zoom_level: f32,
    pub selected_cell: Option<Position>,
    pub highlighted_cells: Vec<Position>,
    pub show_emotions: bool,
    pub show_rules: bool,
    pub show_fitness: bool,
    pub show_connections: bool,
    pub animation_speed: f32,
}

pub struct InteractiveVisualization {
    pub state: VisualizationState,
    pub cell_inspector: CellInspector,
    pub rule_painter: RulePainter,
    pub entity_placer: EntityPlacer,
    pub debug_overlays: DebugOverlays,
}
```

### Cell Inspector System

```rust
#[derive(Clone, Debug)]
pub struct CellInspector {
    pub selected_cell: Option<Position>,
    pub inspection_history: Vec<CellSnapshot>,
    pub show_neighbors: bool,
    pub show_rule_trace: bool,
}

#[derive(Clone, Debug)]
pub struct CellSnapshot {
    pub position: Position,
    pub timestamp: u64,
    pub emotions: [f32; 9],
    pub rule_program: StringBasedGeneticProgram,
    pub fitness_score: f32,
    pub generation: u32,
    pub breeding_history: Vec<BreedingEvent>,
    pub rule_execution_trace: Vec<RuleExecutionStep>,
}

#[derive(Clone, Debug)]
pub struct RuleExecutionStep {
    pub rule_string: String,
    pub step_result: f32,
    pub neighbor_states: Vec<NeighborState>,
    pub execution_time_ms: f32,
}

impl CellInspector {
    pub fn inspect_cell(&mut self, position: Position, grid: &Grid) -> CellSnapshot {
        let cell = &grid.cells[position.y][position.x];
        
        CellSnapshot {
            position,
            timestamp: current_time_ms(),
            emotions: cell.emotions,
            rule_program: cell.rule_program.clone(),
            fitness_score: cell.fitness_score,
            generation: cell.generation,
            breeding_history: cell.breeding_history.clone(),
            rule_execution_trace: self.trace_rule_execution(cell, grid),
        }
    }
    
    fn trace_rule_execution(&self, cell: &Cell, grid: &Grid) -> Vec<RuleExecutionStep> {
        let mut trace = Vec::new();
        
        for rule_string in &cell.rule_program.rule_strings {
            let start_time = precise_time_ms();
            let rule = RuleString::new(rule_string).unwrap();
            let result = RuleParser::evaluate(&rule.compiled_rule, cell, grid);
            let execution_time = precise_time_ms() - start_time;
            
            trace.push(RuleExecutionStep {
                rule_string: rule_string.clone(),
                step_result: result,
                neighbor_states: self.capture_neighbor_states(cell, grid),
                execution_time_ms: execution_time,
            });
        }
        
        trace
    }
    
    pub fn render_inspection_panel(&self, ui: &mut egui::Ui) {
        if let Some(snapshot) = self.inspection_history.last() {
            ui.heading(format!("Cell ({}, {})", snapshot.position.x, snapshot.position.y));
            
            // Emotion display
            ui.collapsing("Emotions", |ui| {
                for (i, &emotion_level) in snapshot.emotions.iter().enumerate() {
                    let emotion_name = EMOTION_NAMES[i];
                    ui.horizontal(|ui| {
                        ui.label(emotion_name);
                        ui.add(egui::ProgressBar::new(emotion_level).text(format!("{:.3}", emotion_level)));
                    });
                }
            });
            
            // Rule display with execution trace
            ui.collapsing("Genetic Program", |ui| {
                ui.label(format!("Combinator: {}", snapshot.rule_program.combinator));
                
                for (i, rule_string) in snapshot.rule_program.rule_strings.iter().enumerate() {
                    ui.collapsing(format!("Rule {}: {}", i, rule_string), |ui| {
                        if let Some(execution_step) = snapshot.rule_execution_trace.get(i) {
                            ui.label(format!("Result: {:.3}", execution_step.step_result));
                            ui.label(format!("Execution time: {:.2}ms", execution_step.execution_time_ms));
                            
                            // Show neighbor states that influenced this rule
                            ui.collapsing("Neighbor States", |ui| {
                                for neighbor_state in &execution_step.neighbor_states {
                                    ui.label(format!("{:?}: {:?}", neighbor_state.position, neighbor_state.emotions));
                                }
                            });
                        }
                    });
                }
            });
            
            // Breeding history
            ui.collapsing("Breeding History", |ui| {
                for breeding_event in &snapshot.breeding_history {
                    ui.label(format!("Gen {}: Bred with ({}, {})", 
                             breeding_event.generation,
                             breeding_event.partner_position.0,
                             breeding_event.partner_position.1));
                }
            });
            
            // Fitness and metadata
            ui.collapsing("Metadata", |ui| {
                ui.label(format!("Fitness: {:.3}", snapshot.fitness_score));
                ui.label(format!("Generation: {}", snapshot.generation));
                ui.label(format!("Age: {} steps", current_time_ms() - snapshot.timestamp));
            });
        }
    }
}
```

### Rule Painting System

```rust
#[derive(Clone, Debug)]
pub struct RulePainter {
    pub brush_mode: BrushMode,
    pub selected_rule: String,
    pub brush_size: u32,
    pub paint_preview: Vec<Position>,
    pub rule_library: Vec<RuleTemplate>,
}

#[derive(Clone, Debug)]
pub enum BrushMode {
    Single,           // Paint one cell at a time
    Circle(u32),      // Paint circular area
    Rectangle(u32, u32), // Paint rectangular area
    Line,             // Paint lines between clicks
    FloodFill,        // Fill connected area with same rule
}

#[derive(Clone, Debug)]
pub struct RuleTemplate {
    pub name: String,
    pub description: String,
    pub rule_strings: Vec<String>,
    pub combinator: String,
    pub color: Color,
    pub preview_icon: String,
}

impl RulePainter {
    pub fn paint_cells(&mut self, positions: &[Position], grid: &mut Grid) {
        let rule_program = StringBasedGeneticProgram {
            rule_strings: vec![self.selected_rule.clone()],
            combinator: "and".to_string(),
        };
        
        for &position in positions {
            if let Some(cell) = grid.get_cell_mut(position) {
                cell.rule_program = rule_program.clone();
                cell.generation = 0; // Reset generation for painted cells
            }
        }
    }
    
    pub fn render_rule_palette(&mut self, ui: &mut egui::Ui) {
        ui.heading("Rule Palette");
        
        // Brush mode selection
        ui.horizontal(|ui| {
            ui.label("Brush:");
            ui.selectable_value(&mut self.brush_mode, BrushMode::Single, "Single");
            ui.selectable_value(&mut self.brush_mode, BrushMode::Circle(3), "Circle");
            ui.selectable_value(&mut self.brush_mode, BrushMode::FloodFill, "Fill");
        });
        
        // Rule templates
        ui.separator();
        ui.label("Rule Templates:");
        
        for template in &self.rule_library {
            ui.horizontal(|ui| {
                let is_selected = self.selected_rule == template.rule_strings[0];
                if ui.selectable_label(is_selected, &template.name).clicked() {
                    self.selected_rule = template.rule_strings[0].clone();
                }
                ui.colored_label(template.color, template.preview_icon.clone());
            });
            
            if ui.is_item_hovered() {
                egui::show_tooltip_at_pointer(ui.ctx(), egui::Id::new("rule_tooltip"), |ui| {
                    ui.label(&template.description);
                    for rule in &template.rule_strings {
                        ui.label(rule);
                    }
                });
            }
        }
        
        // Custom rule input
        ui.separator();
        ui.label("Custom Rule:");
        ui.text_edit_multiline(&mut self.selected_rule);
        
        if ui.button("Test Rule").clicked() {
            // Validate and preview rule
            match RuleString::new(&self.selected_rule) {
                Ok(_) => ui.label("✓ Valid rule"),
                Err(e) => ui.colored_label(egui::Color32::RED, format!("✗ Error: {:?}", e)),
            };
        }
    }
    
    pub fn create_default_rule_library() -> Vec<RuleTemplate> {
        vec![
            RuleTemplate {
                name: "Conway Classic".to_string(),
                description: "Classic Conway's Game of Life".to_string(),
                rule_strings: vec!["count(all, live) == 2 or count(all, live) == 3".to_string()],
                combinator: "and".to_string(),
                color: Color::WHITE,
                preview_icon: "⬜".to_string(),
            },
            RuleTemplate {
                name: "Joy Spreader".to_string(),
                description: "Spreads joy to nearby cells".to_string(),
                rule_strings: vec!["count(all, joy) >= 2 and count(all, joy) <= 4".to_string()],
                combinator: "and".to_string(),
                color: Color::YELLOW,
                preview_icon: "😊".to_string(),
            },
            RuleTemplate {
                name: "Love Generator".to_string(),
                description: "Creates love from concentrated joy".to_string(),
                rule_strings: vec![
                    "north(3, joy) > 0.6".to_string(),
                    "self.joy > 0.5".to_string(),
                ],
                combinator: "and".to_string(),
                color: Color::PINK,
                preview_icon: "💕".to_string(),
            },
            RuleTemplate {
                name: "Anxiety Cloud".to_string(),
                description: "Spreads anxiety rapidly".to_string(),
                rule_strings: vec!["count(all, anxiety) >= 1 or count(all, fear) >= 2".to_string()],
                combinator: "or".to_string(),
                color: Color::PURPLE,
                preview_icon: "😰".to_string(),
            },
            RuleTemplate {
                name: "Social Butterfly".to_string(),
                description: "Thrives with many emotional neighbors".to_string(),
                rule_strings: vec![
                    "sum(all, any) > 3.0".to_string(),
                    "count(all, joy) >= 1".to_string(),
                ],
                combinator: "and".to_string(),
                color: Color::ORANGE,
                preview_icon: "🦋".to_string(),
            },
        ]
    }
}
```

### Entity Placement System

```rust
#[derive(Clone, Debug)]
pub struct EntityPlacer {
    pub placement_mode: PlacementMode,
    pub selected_entity_type: EntityType,
    pub entity_templates: Vec<EntityTemplate>,
    pub preview_positions: Vec<Position>,
}

#[derive(Clone, Debug)]
pub enum PlacementMode {
    Single,
    Random(u32),      // Place N random entities
    Pattern(PlacementPattern),
    Interactive,      // Click to place
}

#[derive(Clone, Debug)]
pub enum PlacementPattern {
    Grid(u32, u32),   // Regular grid spacing
    Circle(Position, u32), // Circle around center
    Line(Position, Position), // Line between points
    Cluster(Position, u32, u32), // Cluster around position
}

impl EntityPlacer {
    pub fn place_entity(&self, position: Position, grid: &mut Grid) {
        let entity_template = &self.entity_templates[0]; // Selected template
        
        if let Some(cell) = grid.get_cell_mut(position) {
            // Apply entity template to cell
            cell.emotions = entity_template.initial_emotions;
            cell.rule_program = entity_template.rule_program.clone();
            cell.fitness_score = 1.0;
            cell.generation = 0;
        }
    }
    
    pub fn render_entity_palette(&mut self, ui: &mut egui::Ui) {
        ui.heading("Entity Placement");
        
        // Placement mode
        ui.horizontal(|ui| {
            ui.label("Mode:");
            ui.selectable_value(&mut self.placement_mode, PlacementMode::Single, "Single");
            ui.selectable_value(&mut self.placement_mode, PlacementMode::Random(10), "Random");
            ui.selectable_value(&mut self.placement_mode, PlacementMode::Interactive, "Interactive");
        });
        
        // Entity templates
        ui.separator();
        for (i, template) in self.entity_templates.iter().enumerate() {
            if ui.selectable_label(false, &template.name).clicked() {
                // Select this template
            }
            ui.small(&template.description);
        }
        
        if ui.button("Clear All Entities").clicked() {
            // Reset all cells to default state
        }
    }
}
```

### Debug Overlays

```rust
#[derive(Clone, Debug)]
pub struct DebugOverlays {
    pub show_emotion_heatmap: bool,
    pub show_fitness_overlay: bool,
    pub show_rule_activity: bool,
    pub show_breeding_connections: bool,
    pub show_genetic_lineages: bool,
    pub emotion_filter: Option<EmotionType>,
}

impl DebugOverlays {
    pub fn render_grid_overlay(&self, grid: &Grid, canvas: &mut Canvas) {
        if self.show_emotion_heatmap {
            self.render_emotion_heatmap(grid, canvas);
        }
        
        if self.show_fitness_overlay {
            self.render_fitness_overlay(grid, canvas);
        }
        
        if self.show_rule_activity {
            self.render_rule_activity(grid, canvas);
        }
        
        if self.show_breeding_connections {
            self.render_breeding_connections(grid, canvas);
        }
    }
    
    fn render_emotion_heatmap(&self, grid: &Grid, canvas: &mut Canvas) {
        for (y, row) in grid.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let emotion_intensity = if let Some(emotion_type) = &self.emotion_filter {
                    cell.emotions[emotion_type.index()]
                } else {
                    // Show dominant emotion
                    cell.emotions.iter().fold(0.0, |max, &val| max.max(val))
                };
                
                let color = self.emotion_to_color(emotion_intensity, &self.emotion_filter);
                canvas.draw_rect(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE, CELL_SIZE, CELL_SIZE, color);
            }
        }
    }
    
    fn render_fitness_overlay(&self, grid: &Grid, canvas: &mut Canvas) {
        for (y, row) in grid.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let alpha = (cell.fitness_score * 255.0) as u8;
                let color = Color::rgba(0, 255, 0, alpha); // Green with fitness-based alpha
                canvas.draw_circle(
                    x as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                    y as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                    CELL_SIZE / 4.0,
                    color
                );
            }
        }
    }
    
    fn render_breeding_connections(&self, grid: &Grid, canvas: &mut Canvas) {
        for (y, row) in grid.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                for breeding_event in &cell.breeding_history {
                    let partner_pos = breeding_event.partner_position;
                    
                    // Draw line between breeding partners
                    canvas.draw_line(
                        x as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                        y as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                        partner_pos.0 as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                        partner_pos.1 as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                        Color::rgba(255, 255, 255, 100), // Semi-transparent white
                        2.0
                    );
                }
            }
        }
    }
}
```

### Zoom and Navigation System

```rust
pub struct ViewportController {
    pub camera_position: Vec2,
    pub zoom_level: f32,
    pub target_zoom: f32,
    pub smooth_zoom: bool,
    pub min_zoom: f32,
    pub max_zoom: f32,
}

impl ViewportController {
    pub fn handle_input(&mut self, input: &InputState) {
        // Mouse wheel zoom
        if input.scroll_delta != 0.0 {
            self.target_zoom *= 1.0 + input.scroll_delta * 0.1;
            self.target_zoom = self.target_zoom.clamp(self.min_zoom, self.max_zoom);
        }
        
        // Pan with middle mouse or arrow keys
        if input.middle_mouse_held || input.arrow_keys_held {
            let pan_speed = 10.0 / self.zoom_level;
            self.camera_position += input.pan_delta * pan_speed;
        }
        
        // Smooth zoom interpolation
        if self.smooth_zoom {
            self.zoom_level = lerp(self.zoom_level, self.target_zoom, 0.1);
        } else {
            self.zoom_level = self.target_zoom;
        }
    }
    
    pub fn screen_to_grid(&self, screen_pos: Vec2) -> Option<Position> {
        let world_pos = (screen_pos - self.camera_position) / self.zoom_level;
        let grid_x = (world_pos.x / CELL_SIZE) as i32;
        let grid_y = (world_pos.y / CELL_SIZE) as i32;
        
        if grid_x >= 0 && grid_y >= 0 {
            Some(Position { x: grid_x as usize, y: grid_y as usize })
        } else {
            None
        }
    }
    
    pub fn focus_on_cell(&mut self, position: Position) {
        self.camera_position = Vec2::new(
            position.x as f32 * CELL_SIZE * self.zoom_level,
            position.y as f32 * CELL_SIZE * self.zoom_level
        );
    }
}
```

This visualization system creates a comprehensive debugging and interaction environment where you can:

1. **Zoom from macro to micro**: See system patterns and individual cell behavior
2. **Interactive debugging**: Click cells to inspect emotions, rules, and execution traces  
3. **Visual rule editing**: Paint cells with different genetic programs
4. **Real-time monitoring**: Watch emotions and rules evolve in real-time
5. **Genetic lineage tracking**: See how successful traits spread through breeding
6. **Rule library**: Pre-built and custom rule templates for experimentation

The string-based rules combined with rich visualization makes genetic programming accessible and comprehensible!

## High-Performance Rule Execution Engine

### Bytecode-Based Virtual Machine

Instead of Lua (too slow) or native compilation (too complex), we use a custom bytecode VM optimized for cellular automata operations.

```rust
#[derive(Clone, Debug)]
pub enum ByteCode {
    // Stack operations
    LoadConst(f32),
    LoadSelf(EmotionIndex),
    LoadNeighbor(Direction, EmotionIndex),
    LoadNeighborCount(Direction, EmotionIndex, f32), // direction, emotion, threshold
    
    // Arithmetic operations
    Add,
    Sub,
    Mul,
    Div,
    
    // Comparison operations
    Greater,
    Less,
    Equal,
    GreaterEqual,
    LessEqual,
    
    // Logic operations
    And,
    Or,
    Not,
    
    // Specialized CA operations (highly optimized)
    CountNeighbors(NeighborMask, EmotionIndex, f32), // mask, emotion, threshold
    SumNeighbors(NeighborMask, EmotionIndex),
    PatternMatch(PatternId),
    
    // Control flow
    Jump(usize),
    JumpIfFalse(usize),
    
    // Stack manipulation
    Dup,
    Pop,
    
    // Result
    Return,
}

#[derive(Clone, Debug)]
pub struct CompiledRule {
    pub bytecode: Vec<ByteCode>,
    pub constants: Vec<f32>,
    pub neighbor_masks: Vec<NeighborMask>, // Pre-computed neighbor access patterns
    pub patterns: Vec<PrecomputedPattern>,
}

// Ultra-fast neighbor access using bit masks
#[derive(Clone, Debug)]
pub struct NeighborMask {
    pub mask: u8, // 8 bits for 8 neighbors (N, NE, E, SE, S, SW, W, NW)
    pub offsets: Vec<(i8, i8)>, // Pre-computed relative positions
}

impl NeighborMask {
    pub const NORTH: u8 = 0b00000001;
    pub const NORTHEAST: u8 = 0b00000010;
    pub const EAST: u8 = 0b00000100;
    pub const SOUTHEAST: u8 = 0b00001000;
    pub const SOUTH: u8 = 0b00010000;
    pub const SOUTHWEST: u8 = 0b00100000;
    pub const WEST: u8 = 0b01000000;
    pub const NORTHWEST: u8 = 0b10000000;
    
    pub const ALL: u8 = 0b11111111;
    pub const CARDINAL: u8 = 0b01010101; // N, E, S, W
    pub const DIAGONAL: u8 = 0b10101010; // NE, SE, SW, NW
    
    pub fn north(count: usize) -> Self {
        // Create mask for N northernmost neighbors
        let mut mask = 0;
        let mut offsets = Vec::new();
        
        for i in 1..=count.min(8) {
            mask |= Self::NORTH;
            offsets.push((0, -(i as i8)));
        }
        
        Self { mask, offsets }
    }
    
    pub fn custom(positions: &[(i8, i8)]) -> Self {
        let offsets = positions.to_vec();
        // Convert positions to bitmask if they match standard neighbors
        let mut mask = 0;
        // ... implementation
        Self { mask, offsets }
    }
}
```

### Ultra-Fast Rule Compiler

```rust
pub struct RuleCompiler {
    pub neighbor_cache: HashMap<String, NeighborMask>,
    pub pattern_cache: HashMap<String, PrecomputedPattern>,
}

impl RuleCompiler {
    pub fn compile(rule_text: &str) -> Result<CompiledRule, CompileError> {
        let tokens = Self::tokenize(rule_text)?;
        let ast = Self::parse_tokens(tokens)?;
        let bytecode = Self::compile_ast(ast)?;
        
        Ok(CompiledRule {
            bytecode,
            constants: Vec::new(),
            neighbor_masks: Vec::new(),
            patterns: Vec::new(),
        })
    }
    
    fn compile_ast(ast: ASTNode) -> Result<Vec<ByteCode>, CompileError> {
        let mut bytecode = Vec::new();
        
        match ast {
            ASTNode::FunctionCall(name, args) => {
                match name.as_str() {
                    "north" => {
                        // north(4, joy) -> CountNeighbors with north mask
                        let count = args[0].as_number()?;
                        let emotion = args[1].as_emotion()?;
                        let mask = NeighborMask::north(count as usize);
                        bytecode.push(ByteCode::CountNeighbors(mask, emotion, 0.5));
                    },
                    "count" => {
                        // count(all, anger) -> CountNeighbors with all mask
                        let direction = args[0].as_direction()?;
                        let emotion = args[1].as_emotion()?;
                        let mask = direction.to_neighbor_mask();
                        bytecode.push(ByteCode::CountNeighbors(mask, emotion, 0.5));
                    },
                    "sum" => {
                        let direction = args[0].as_direction()?;
                        let emotion = args[1].as_emotion()?;
                        let mask = direction.to_neighbor_mask();
                        bytecode.push(ByteCode::SumNeighbors(mask, emotion));
                    },
                    "self" => {
                        let emotion = args[0].as_emotion()?;
                        bytecode.push(ByteCode::LoadSelf(emotion));
                    },
                    _ => return Err(CompileError::UnknownFunction(name)),
                }
            },
            ASTNode::BinaryOp(op, left, right) => {
                // Compile left side, compile right side, then operation
                bytecode.extend(Self::compile_ast(*left)?);
                bytecode.extend(Self::compile_ast(*right)?);
                
                match op {
                    BinaryOperator::Greater => bytecode.push(ByteCode::Greater),
                    BinaryOperator::Less => bytecode.push(ByteCode::Less),
                    BinaryOperator::Equal => bytecode.push(ByteCode::Equal),
                    BinaryOperator::And => bytecode.push(ByteCode::And),
                    BinaryOperator::Or => bytecode.push(ByteCode::Or),
                    BinaryOperator::Add => bytecode.push(ByteCode::Add),
                    BinaryOperator::Sub => bytecode.push(ByteCode::Sub),
                    BinaryOperator::Mul => bytecode.push(ByteCode::Mul),
                    BinaryOperator::Div => bytecode.push(ByteCode::Div),
                }
            },
            ASTNode::Number(n) => {
                bytecode.push(ByteCode::LoadConst(n));
            },
            // ... other AST nodes
        }
        
        bytecode.push(ByteCode::Return);
        Ok(bytecode)
    }
}
```

### High-Performance Virtual Machine

```rust
pub struct CellularAutomataVM {
    stack: Vec<f32>,
    grid_cache: GridCache, // Pre-computed neighbor access
}

#[derive(Clone)]
pub struct GridCache {
    pub cells: *const Cell, // Raw pointer for maximum speed
    pub width: usize,
    pub height: usize,
    pub neighbor_lookup: Vec<Vec<Option<usize>>>, // Pre-computed neighbor indices
}

impl CellularAutomataVM {
    pub fn execute(&mut self, rule: &CompiledRule, cell_index: usize) -> f32 {
        self.stack.clear();
        let mut pc = 0; // Program counter
        
        while pc < rule.bytecode.len() {
            match &rule.bytecode[pc] {
                ByteCode::LoadConst(val) => {
                    self.stack.push(*val);
                },
                
                ByteCode::LoadSelf(emotion_idx) => {
                    let cell = unsafe { &*self.grid_cache.cells.add(cell_index) };
                    self.stack.push(cell.emotions[*emotion_idx as usize]);
                },
                
                ByteCode::CountNeighbors(mask, emotion_idx, threshold) => {
                    let count = self.count_neighbors_fast(cell_index, mask, *emotion_idx, *threshold);
                    self.stack.push(count as f32);
                },
                
                ByteCode::SumNeighbors(mask, emotion_idx) => {
                    let sum = self.sum_neighbors_fast(cell_index, mask, *emotion_idx);
                    self.stack.push(sum);
                },
                
                ByteCode::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                },
                
                ByteCode::Greater => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(if a > b { 1.0 } else { 0.0 });
                },
                
                ByteCode::And => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(if a > 0.5 && b > 0.5 { 1.0 } else { 0.0 });
                },
                
                ByteCode::Return => {
                    return self.stack.pop().unwrap_or(0.0);
                },
                
                _ => {
                    // Handle other bytecode instructions
                }
            }
            
            pc += 1;
        }
        
        0.0
    }
    
    #[inline(always)]
    fn count_neighbors_fast(&self, cell_index: usize, mask: &NeighborMask, emotion_idx: EmotionIndex, threshold: f32) -> u32 {
        let mut count = 0;
        
        // Use pre-computed neighbor lookup for maximum speed
        if let Some(neighbor_indices) = self.grid_cache.neighbor_lookup.get(cell_index) {
            for (i, &neighbor_idx_opt) in neighbor_indices.iter().enumerate() {
                if (mask.mask & (1 << i)) != 0 {
                    if let Some(neighbor_idx) = neighbor_idx_opt {
                        let neighbor = unsafe { &*self.grid_cache.cells.add(neighbor_idx) };
                        if neighbor.emotions[emotion_idx as usize] > threshold {
                            count += 1;
                        }
                    }
                }
            }
        }
        
        count
    }
    
    #[inline(always)]
    fn sum_neighbors_fast(&self, cell_index: usize, mask: &NeighborMask, emotion_idx: EmotionIndex) -> f32 {
        let mut sum = 0.0;
        
        if let Some(neighbor_indices) = self.grid_cache.neighbor_lookup.get(cell_index) {
            for (i, &neighbor_idx_opt) in neighbor_indices.iter().enumerate() {
                if (mask.mask & (1 << i)) != 0 {
                    if let Some(neighbor_idx) = neighbor_idx_opt {
                        let neighbor = unsafe { &*self.grid_cache.cells.add(neighbor_idx) };
                        sum += neighbor.emotions[emotion_idx as usize];
                    }
                }
            }
        }
        
        sum
    }
}
```

### Performance Optimizations

```rust
// Pre-compile all common rule patterns into native functions
pub struct OptimizedRuleLibrary {
    pub conway_classic: fn(&Cell, &[Option<&Cell>; 8]) -> f32,
    pub joy_spreader: fn(&Cell, &[Option<&Cell>; 8]) -> f32,
    pub love_generator: fn(&Cell, &[Option<&Cell>; 8]) -> f32,
    // ... other optimized rules
}

impl OptimizedRuleLibrary {
    pub fn conway_classic(cell: &Cell, neighbors: &[Option<&Cell>; 8]) -> f32 {
        let live_count = neighbors.iter()
            .filter_map(|n| *n)
            .count();
        
        if live_count == 3 || (cell.is_alive() && live_count == 2) {
            1.0
        } else {
            0.0
        }
    }
    
    pub fn joy_spreader(cell: &Cell, neighbors: &[Option<&Cell>; 8]) -> f32 {
        let joy_count = neighbors.iter()
            .filter_map(|n| *n)
            .filter(|n| n.emotions[EmotionType::Joy as usize] > 0.5)
            .count();
        
        if joy_count >= 2 && joy_count <= 4 {
            1.0
        } else {
            0.0
        }
    }
}

// SIMD-optimized operations for bulk processing
pub mod simd_rules {
    use std::simd::*;
    
    pub fn count_emotions_simd(emotions: &[f32x8], threshold: f32x8) -> u32x8 {
        // Use SIMD to process 8 cells at once
        let mask = emotions[0].simd_gt(threshold);
        mask.to_bitmask().count_ones() as u32x8
    }
    
    pub fn sum_emotions_simd(emotions: &[f32x8]) -> f32x8 {
        emotions.iter().fold(f32x8::splat(0.0), |acc, &x| acc + x)
    }
}
```

### Rule String → Bytecode Examples

```rust
// "north(4, joy) > 0.5" compiles to:
vec![
    ByteCode::CountNeighbors(NeighborMask::north(4), EmotionType::Joy, 0.5),
    ByteCode::LoadConst(0.5),
    ByteCode::Greater,
    ByteCode::Return,
]

// "count(all, anger) + self.joy" compiles to:
vec![
    ByteCode::CountNeighbors(NeighborMask::ALL, EmotionType::Anger, 0.5),
    ByteCode::LoadSelf(EmotionType::Joy),
    ByteCode::Add,
    ByteCode::Return,
]

// "north(3, joy) > 0.6 and self.joy > 0.5" compiles to:
vec![
    ByteCode::CountNeighbors(NeighborMask::north(3), EmotionType::Joy, 0.6),
    ByteCode::LoadConst(0.6),
    ByteCode::Greater,
    ByteCode::LoadSelf(EmotionType::Joy),
    ByteCode::LoadConst(0.5),
    ByteCode::Greater,
    ByteCode::And,
    ByteCode::Return,
]
```

### Performance Characteristics

- **String rules**: Human-readable, easy to mutate genetically
- **Bytecode compilation**: ~10-100x faster than Lua interpretation
- **Pre-computed neighbor lookup**: O(1) neighbor access instead of coordinate math
- **SIMD operations**: Process multiple cells simultaneously
- **Specialized instructions**: Optimized for cellular automata operations
- **Native rule library**: Critical paths compiled to native functions

This gives us:
1. **Human-readable string rules** for genetic programming
2. **Near-native performance** for real-time simulation
3. **Easy mutation and crossover** through bytecode manipulation
4. **Scalability** to large grids with thousands of cells

The best of both worlds - expressive genetic programming with blazing fast execution!

## Distributed Architecture: Engine + UI + External Control

### System Architecture

Split the system into three distinct components communicating over WebSockets:

```rust
// 1. Core Engine (headless simulation)
// 2. UI Client (visualization + interaction)  
// 3. External Controllers (machines, scripts, automation)

┌─────────────────┐    WebSocket    ┌─────────────────┐
│   UI Client     │ ◄─────────────► │  Core Engine    │
│  (macroquad)    │                 │   (headless)    │
└─────────────────┘                 └─────────────────┘
                                            ▲
                                            │ WebSocket
                                            ▼
                                    ┌─────────────────┐
                                    │ External        │
                                    │ Controllers     │
                                    │ (Python/JS/etc) │
                                    └─────────────────┘
```

### Core Engine (Headless)

```rust
// emotion_contagion_engine/src/main.rs
use tokio_tungstenite::WebSocketStream;
use serde::{Serialize, Deserialize};

pub struct EmotionContagionEngine {
    pub simulation: EmotionSimulation,
    pub websocket_server: WebSocketServer,
    pub clients: Vec<ConnectedClient>,
    pub running: bool,
    pub target_fps: u32,
}

#[derive(Clone, Debug)]
pub struct ConnectedClient {
    pub id: String,
    pub client_type: ClientType,
    pub websocket: Arc<Mutex<WebSocketStream>>,
    pub subscriptions: Vec<EventSubscription>,
}

#[derive(Clone, Debug)]
pub enum ClientType {
    UI,           // Visual interface
    Controller,   // External automation
    Monitor,      // Read-only observer
    Researcher,   // Full control + data export
}

impl EmotionContagionEngine {
    pub async fn run(&mut self) {
        let mut last_frame = Instant::now();
        let frame_duration = Duration::from_millis(1000 / self.target_fps as u64);
        
        while self.running {
            // Process incoming WebSocket messages
            self.process_client_messages().await;
            
            // Run simulation step
            if !self.simulation.paused {
                self.simulation.step();
            }
            
            // Broadcast updates to subscribed clients
            self.broadcast_updates().await;
            
            // Frame rate limiting
            let elapsed = last_frame.elapsed();
            if elapsed < frame_duration {
                tokio::time::sleep(frame_duration - elapsed).await;
            }
            last_frame = Instant::now();
        }
    }
    
    async fn process_client_messages(&mut self) {
        for client in &mut self.clients {
            while let Some(message) = self.receive_message(client).await {
                self.handle_message(client, message).await;
            }
        }
    }
    
    async fn handle_message(&mut self, client: &ConnectedClient, message: ClientMessage) {
        match message {
            ClientMessage::SimulationControl(cmd) => {
                self.handle_simulation_command(client, cmd).await;
            },
            ClientMessage::RulePainting(paint_cmd) => {
                self.handle_rule_painting(client, paint_cmd).await;
            },
            ClientMessage::DataQuery(query) => {
                self.handle_data_query(client, query).await;
            },
            ClientMessage::Subscribe(subscription) => {
                self.handle_subscription(client, subscription).await;
            },
            ClientMessage::GeneticProgramming(gp_cmd) => {
                self.handle_genetic_programming(client, gp_cmd).await;
            },
        }
    }
}
```

### WebSocket Message Protocol

```rust
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ClientMessage {
    // Simulation Control
    SimulationControl(SimulationCommand),
    
    // Interactive Editing
    RulePainting(RulePaintCommand),
    EntityPlacement(EntityPlaceCommand),
    
    // Data Access
    DataQuery(DataQueryCommand),
    Subscribe(SubscriptionCommand),
    
    // Genetic Programming
    GeneticProgramming(GeneticProgrammingCommand),
    
    // Authentication
    Authenticate(AuthCommand),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum SimulationCommand {
    Start,
    Pause,
    Step { count: u32 },
    Reset,
    SetSpeed { fps: u32 },
    SetGridSize { width: usize, height: usize },
    LoadConfiguration { config: SimulationConfig },
    SaveConfiguration { filename: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum RulePaintCommand {
    PaintCells { 
        positions: Vec<Position>, 
        rule_string: String,
        brush_mode: BrushMode,
    },
    CreateRuleTemplate {
        name: String,
        rule_strings: Vec<String>,
        combinator: String,
    },
    GetRuleLibrary,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum DataQueryCommand {
    GetCellState { position: Position },
    GetGridState { region: Option<Region> },
    GetFitnessMetrics,
    GetEvolutionStats,
    GetRuleDistribution,
    ExportData { format: ExportFormat, filename: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum GeneticProgrammingCommand {
    SetMutationRate { rate: f32 },
    SetCrossoverRate { rate: f32 },
    InjectRule { rule_string: String, positions: Vec<Position> },
    TriggerEvolution { generations: u32 },
    SetFitnessFunction { function_type: FitnessFunction },
    GetGeneticStats,
}

// Server to Client Messages
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ServerMessage {
    // Real-time updates
    GridUpdate(GridUpdateMessage),
    CellUpdate(CellUpdateMessage),
    EvolutionEvent(EvolutionEventMessage),
    
    // Response messages
    QueryResponse(QueryResponseMessage),
    CommandResult(CommandResultMessage),
    
    // Status updates
    StatusUpdate(StatusUpdateMessage),
    Error(ErrorMessage),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GridUpdateMessage {
    pub timestamp: u64,
    pub generation: u32,
    pub grid_state: GridState, // Compressed representation
    pub changed_cells: Vec<Position>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CellUpdateMessage {
    pub position: Position,
    pub emotions: [f32; 9],
    pub rule_program: String, // Serialized rule
    pub fitness: f32,
    pub generation: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvolutionEventMessage {
    pub event_type: EvolutionEventType,
    pub generation: u32,
    pub participants: Vec<Position>,
    pub new_rule: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EvolutionEventType {
    Breeding,
    Mutation,
    Death,
    Birth,
    FitnessImprovement,
}
```

### UI Client (Separate Process)

```rust
// emotion_contagion_ui/src/main.rs
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub struct EmotionContagionUI {
    pub websocket_client: WebSocketClient,
    pub visualization: InteractiveVisualization,
    pub local_state: UIState,
    pub connection_status: ConnectionStatus,
}

impl EmotionContagionUI {
    pub async fn connect_to_engine(&mut self, engine_url: &str) -> Result<(), Box<dyn Error>> {
        let (ws_stream, _) = connect_async(engine_url).await?;
        self.websocket_client = WebSocketClient::new(ws_stream);
        
        // Subscribe to grid updates
        self.websocket_client.send(ClientMessage::Subscribe(
            SubscriptionCommand::GridUpdates { frequency: UpdateFrequency::RealTime }
        )).await?;
        
        Ok(())
    }
    
    pub async fn run(&mut self) {
        loop {
            // Handle UI events (mouse, keyboard, etc.)
            self.handle_ui_events().await;
            
            // Process messages from engine
            self.process_engine_messages().await;
            
            // Render frame
            self.render();
            
            next_frame().await;
        }
    }
    
    async fn handle_ui_events(&mut self) {
        // Mouse clicks for cell inspection
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            if let Some(grid_pos) = self.visualization.screen_to_grid(mouse_pos) {
                // Request cell state from engine
                self.websocket_client.send(ClientMessage::DataQuery(
                    DataQueryCommand::GetCellState { position: grid_pos }
                )).await.unwrap();
            }
        }
        
        // Rule painting
        if self.visualization.rule_painter.is_painting() {
            let painted_positions = self.visualization.rule_painter.get_painted_positions();
            let rule_string = self.visualization.rule_painter.selected_rule.clone();
            
            self.websocket_client.send(ClientMessage::RulePainting(
                RulePaintCommand::PaintCells {
                    positions: painted_positions,
                    rule_string,
                    brush_mode: self.visualization.rule_painter.brush_mode.clone(),
                }
            )).await.unwrap();
        }
        
        // Simulation controls
        if is_key_pressed(KeyCode::Space) {
            self.websocket_client.send(ClientMessage::SimulationControl(
                SimulationCommand::Pause
            )).await.unwrap();
        }
    }
    
    async fn process_engine_messages(&mut self) {
        while let Some(message) = self.websocket_client.receive().await {
            match message {
                ServerMessage::GridUpdate(update) => {
                    self.visualization.update_grid_state(update);
                },
                ServerMessage::CellUpdate(update) => {
                    self.visualization.update_cell_state(update);
                },
                ServerMessage::QueryResponse(response) => {
                    self.handle_query_response(response);
                },
                ServerMessage::EvolutionEvent(event) => {
                    self.visualization.show_evolution_event(event);
                },
                _ => {}
            }
        }
    }
}
```

### External Controller API

```python
# Python client example
import asyncio
import websockets
import json

class EmotionContagionController:
    def __init__(self, engine_url="ws://localhost:8080"):
        self.engine_url = engine_url
        self.websocket = None
        
    async def connect(self):
        self.websocket = await websockets.connect(self.engine_url)
        
        # Authenticate as controller
        await self.send_message({
            "type": "Authenticate",
            "client_type": "Controller",
            "capabilities": ["simulation_control", "genetic_programming", "data_export"]
        })
    
    async def send_message(self, message):
        await self.websocket.send(json.dumps(message))
    
    async def receive_message(self):
        message = await self.websocket.recv()
        return json.loads(message)
    
    # High-level automation methods
    async def run_evolution_experiment(self, generations=1000, mutation_rate=0.1):
        """Run automated evolution experiment"""
        
        # Set genetic parameters
        await self.send_message({
            "type": "GeneticProgramming",
            "action": "SetMutationRate",
            "rate": mutation_rate
        })
        
        # Subscribe to evolution events
        await self.send_message({
            "type": "Subscribe",
            "subscription_type": "EvolutionEvents"
        })
        
        # Start simulation
        await self.send_message({
            "type": "SimulationControl", 
            "action": "Start"
        })
        
        # Monitor and collect data
        fitness_history = []
        for generation in range(generations):
            # Wait for generation complete event
            message = await self.receive_message()
            if message["type"] == "EvolutionEvent":
                # Collect fitness metrics
                fitness_data = await self.get_fitness_metrics()
                fitness_history.append(fitness_data)
                
                # Adaptive parameter adjustment
                if fitness_data["average_fitness"] < 0.3:
                    await self.send_message({
                        "type": "GeneticProgramming",
                        "action": "SetMutationRate", 
                        "rate": mutation_rate * 1.5  # Increase exploration
                    })
        
        return fitness_history
    
    async def inject_rule_pattern(self, rule_string, pattern="random", count=10):
        """Inject specific rules into the population"""
        
        if pattern == "random":
            # Get grid size first
            grid_info = await self.get_grid_info()
            positions = self.generate_random_positions(grid_info, count)
        elif pattern == "corners":
            positions = self.generate_corner_positions(grid_info)
        
        await self.send_message({
            "type": "GeneticProgramming",
            "action": "InjectRule",
            "rule_string": rule_string,
            "positions": positions
        })
    
    async def export_evolution_data(self, filename="evolution_data.json"):
        """Export complete evolution dataset"""
        await self.send_message({
            "type": "DataQuery",
            "action": "ExportData",
            "format": "JSON",
            "filename": filename
        })

# Example usage
async def main():
    controller = EmotionContagionController()
    await controller.connect()
    
    # Run systematic experiments
    results = []
    
    for mutation_rate in [0.05, 0.1, 0.15, 0.2]:
        print(f"Testing mutation rate: {mutation_rate}")
        
        # Reset simulation
        await controller.send_message({
            "type": "SimulationControl",
            "action": "Reset"
        })
        
        # Run experiment
        fitness_history = await controller.run_evolution_experiment(
            generations=500, 
            mutation_rate=mutation_rate
        )
        
        results.append({
            "mutation_rate": mutation_rate,
            "fitness_history": fitness_history,
            "final_fitness": fitness_history[-1]["average_fitness"]
        })
    
    # Find optimal parameters
    best_result = max(results, key=lambda x: x["final_fitness"])
    print(f"Best mutation rate: {best_result['mutation_rate']}")
    
    # Export all data
    await controller.export_evolution_data("experiment_results.json")

if __name__ == "__main__":
    asyncio.run(main())
```

### REST API for Simple Operations

```rust
use warp::Filter;

pub struct RestApiServer {
    pub simulation: Arc<Mutex<EmotionSimulation>>,
}

impl RestApiServer {
    pub async fn start(&self, port: u16) {
        let simulation = self.simulation.clone();
        
        // GET /status
        let status = warp::path("status")
            .and(warp::get())
            .and_then(move || {
                let sim = simulation.clone();
                async move {
                    let sim = sim.lock().await;
                    Ok::<_, warp::Rejection>(warp::reply::json(&json!({
                        "running": !sim.paused,
                        "generation": sim.generation,
                        "fps": sim.actual_fps,
                        "population_size": sim.entities.len(),
                    })))
                }
            });
        
        // POST /simulation/start
        let start = warp::path!("simulation" / "start")
            .and(warp::post())
            .and_then(move || {
                let sim = simulation.clone();
                async move {
                    let mut sim = sim.lock().await;
                    sim.paused = false;
                    Ok::<_, warp::Rejection>(warp::reply::json(&json!({
                        "status": "started"
                    })))
                }
            });
        
        // GET /cells/{x}/{y}
        let get_cell = warp::path!("cells" / usize / usize)
            .and(warp::get())
            .and_then(move |x, y| {
                let sim = simulation.clone();
                async move {
                    let sim = sim.lock().await;
                    if let Some(cell) = sim.grid.get_cell(Position { x, y }) {
                        Ok::<_, warp::Rejection>(warp::reply::json(&json!({
                            "position": {"x": x, "y": y},
                            "emotions": cell.emotions,
                            "rule_program": cell.rule_program,
                            "fitness": cell.fitness_score,
                            "generation": cell.generation,
                        })))
                    } else {
                        Err(warp::reject::not_found())
                    }
                }
            });
        
        let routes = status.or(start).or(get_cell);
        
        warp::serve(routes)
            .run(([127, 0, 0, 1], port))
            .await;
    }
}
```

### Benefits of Distributed Architecture

1. **Separation of Concerns**: Engine focuses on simulation, UI focuses on visualization
2. **Multiple Clients**: Multiple UIs, controllers, and monitors can connect simultaneously
3. **Language Agnostic**: External controllers can be written in any language
4. **Scalability**: Engine can run on powerful hardware, UI on different machines
5. **Automation**: Perfect for systematic research and automated experiments
6. **Integration**: Easy to integrate with ML pipelines, databases, and research tools

This architecture turns the emotion contagion system into a proper research platform that machines can control and study systematically!

## Hybrid Architecture: Performance + Control

### Optimal Architecture Design

Based on performance analysis, the best approach combines integrated UI for visualization with WebSocket control for automation:

```rust
┌─────────────────────────────────────────────────────────────┐
│                Emotion Contagion Application                │
│                                                             │
│  ┌─────────────────┐    ┌─────────────────┐                │
│  │   Core Engine   │◄──►│   Integrated    │                │
│  │   (Bytecode VM) │    │   UI (macroquad)│                │
│  │                 │    │                 │                │
│  │ • Grid simulation│    │ • Real-time viz │                │
│  │ • Genetic rules │    │ • Cell inspector│                │
│  │ • Evolution     │    │ • Rule painting │                │
│  │ • Fast execution│    │ • Zoom/pan      │                │
│  └─────────────────┘    └─────────────────┘                │
│            ▲                                                │
│            │                                                │
│  ┌─────────▼─────────┐                                      │
│  │  WebSocket Server │                                      │
│  │  (Control API)    │                                      │
│  └─────────┬─────────┘                                      │
└────────────┼─────────────────────────────────────────────────┘
             │
    ┌────────▼────────┐         ┌─────────────────┐
    │ External        │         │ Web Dashboard   │
    │ Controllers     │         │ (HTML/JS)       │
    │ (Python/etc)    │         │                 │
    │                 │         │ • Population    │
    │ • Automation    │         │   stats         │
    │ • Experiments   │         │ • Rule analysis │
    │ • Data export   │         │ • Fitness plots │
    │ • ML integration│         │ • Export tools  │
    └─────────────────┘         └─────────────────┘
```

### Application Structure

```rust
// Main application - single binary
pub struct EmotionContagionApp {
    pub engine: EmotionSimulation,
    pub ui: IntegratedUI,
    pub websocket_server: ControlServer,
    pub web_dashboard: DashboardServer,
}

impl EmotionContagionApp {
    pub async fn run(&mut self) {
        // Start WebSocket control server
        tokio::spawn(async move {
            self.websocket_server.start(8080).await;
        });
        
        // Start web dashboard server  
        tokio::spawn(async move {
            self.web_dashboard.start(8081).await;
        });
        
        // Main visualization loop (60 FPS)
        loop {
            // Process control commands from WebSocket
            self.process_control_commands();
            
            // Update simulation
            if !self.engine.paused {
                self.engine.step();
            }
            
            // Render UI (direct access - no network overhead)
            self.ui.render(&self.engine);
            
            // Send updates to web clients (throttled)
            if self.should_send_web_update() {
                self.send_dashboard_update().await;
            }
            
            next_frame().await;
        }
    }
}
```

### WebSocket Control API (Simplified)

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ControlMessage {
    // Simulation control
    StartSimulation,
    PauseSimulation, 
    ResetSimulation,
    SetSpeed { fps: u32 },
    
    // Genetic programming
    SetMutationRate { rate: f32 },
    InjectRule { rule: String, positions: Vec<Position> },
    TriggerEvolution { generations: u32 },
    
    // Data queries (non-real-time)
    GetFitnessStats,
    GetRuleDistribution,
    GetPopulationMetrics,
    ExportData { format: String },
    
    // Configuration
    LoadConfig { filename: String },
    SaveConfig { filename: String },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ControlResponse {
    Status { running: bool, generation: u32, fps: f32 },
    FitnessStats { average: f32, max: f32, distribution: Vec<f32> },
    RuleDistribution { rules: HashMap<String, u32> },
    PopulationMetrics { total_cells: u32, active_cells: u32, diversity_index: f32 },
    DataExported { filename: String, size_bytes: u64 },
    Error { message: String },
}

// Control server - high-level commands only
pub struct ControlServer {
    pub simulation: Arc<Mutex<EmotionSimulation>>,
    pub clients: Vec<ControlClient>,
}

impl ControlServer {
    pub async fn handle_message(&mut self, message: ControlMessage) -> ControlResponse {
        match message {
            ControlMessage::StartSimulation => {
                let mut sim = self.simulation.lock().unwrap();
                sim.paused = false;
                ControlResponse::Status { 
                    running: true, 
                    generation: sim.generation,
                    fps: sim.actual_fps 
                }
            },
            ControlMessage::SetMutationRate { rate } => {
                let mut sim = self.simulation.lock().unwrap();
                sim.evolution_engine.mutation_rate = rate;
                ControlResponse::Status { 
                    running: !sim.paused,
                    generation: sim.generation,
                    fps: sim.actual_fps 
                }
            },
            ControlMessage::GetFitnessStats => {
                let sim = self.simulation.lock().unwrap();
                let fitness_values: Vec<f32> = sim.grid.cells
                    .iter()
                    .flatten()
                    .map(|cell| cell.fitness_score)
                    .collect();
                
                let average = fitness_values.iter().sum::<f32>() / fitness_values.len() as f32;
                let max = fitness_values.iter().fold(0.0, |a, &b| a.max(b));
                
                ControlResponse::FitnessStats { average, max, distribution: fitness_values }
            },
            // ... other commands
        }
    }
}
```

### Web Dashboard (HTML/JS)

```html
<!-- web_dashboard/index.html -->
<!DOCTYPE html>
<html>
<head>
    <title>Emotion Contagion Dashboard</title>
    <script src="https://cdn.plot.ly/plotly-latest.min.js"></script>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .controls { margin-bottom: 20px; }
        .metrics { display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 20px; }
        .chart { height: 300px; border: 1px solid #ddd; margin-bottom: 20px; }
        .rule-list { max-height: 400px; overflow-y: auto; }
    </style>
</head>
<body>
    <h1>Emotion Contagion Control Dashboard</h1>
    
    <div class="controls">
        <button id="start">Start</button>
        <button id="pause">Pause</button>
        <button id="reset">Reset</button>
        <label>Speed: <input type="range" id="speed" min="1" max="120" value="60"></label>
        <label>Mutation Rate: <input type="range" id="mutation" min="0" max="0.5" step="0.01" value="0.1"></label>
    </div>
    
    <div class="metrics">
        <div>
            <h3>Population Stats</h3>
            <div id="population-stats">
                <p>Generation: <span id="generation">0</span></p>
                <p>Active Cells: <span id="active-cells">0</span></p>
                <p>Avg Fitness: <span id="avg-fitness">0.00</span></p>
                <p>Max Fitness: <span id="max-fitness">0.00</span></p>
            </div>
        </div>
        
        <div>
            <h3>Rule Distribution</h3>
            <div id="rule-chart" class="chart"></div>
        </div>
        
        <div>
            <h3>Fitness Over Time</h3>
            <div id="fitness-chart" class="chart"></div>
        </div>
    </div>
    
    <div>
        <h3>Rule Injection</h3>
        <textarea id="rule-input" placeholder="Enter rule string..." rows="3" cols="50"></textarea>
        <button id="inject-rule">Inject Rule</button>
        <label>Count: <input type="number" id="inject-count" value="10" min="1" max="100"></label>
    </div>
    
    <div>
        <h3>Active Rules</h3>
        <div id="rule-list" class="rule-list"></div>
    </div>

    <script>
        class DashboardController {
            constructor() {
                this.ws = new WebSocket('ws://localhost:8080');
                this.fitnessHistory = [];
                this.setupEventListeners();
                this.startDataPolling();
            }
            
            setupEventListeners() {
                document.getElementById('start').onclick = () => {
                    this.sendCommand({ type: 'StartSimulation' });
                };
                
                document.getElementById('pause').onclick = () => {
                    this.sendCommand({ type: 'PauseSimulation' });
                };
                
                document.getElementById('reset').onclick = () => {
                    this.sendCommand({ type: 'ResetSimulation' });
                    this.fitnessHistory = [];
                };
                
                document.getElementById('speed').oninput = (e) => {
                    this.sendCommand({ 
                        type: 'SetSpeed', 
                        fps: parseInt(e.target.value) 
                    });
                };
                
                document.getElementById('mutation').oninput = (e) => {
                    this.sendCommand({ 
                        type: 'SetMutationRate', 
                        rate: parseFloat(e.target.value) 
                    });
                };
                
                document.getElementById('inject-rule').onclick = () => {
                    const rule = document.getElementById('rule-input').value;
                    const count = parseInt(document.getElementById('inject-count').value);
                    
                    if (rule.trim()) {
                        this.sendCommand({
                            type: 'InjectRule',
                            rule: rule,
                            positions: this.generateRandomPositions(count)
                        });
                    }
                };
            }
            
            sendCommand(command) {
                if (this.ws.readyState === WebSocket.OPEN) {
                    this.ws.send(JSON.stringify(command));
                }
            }
            
            startDataPolling() {
                setInterval(() => {
                    this.sendCommand({ type: 'GetFitnessStats' });
                    this.sendCommand({ type: 'GetRuleDistribution' });
                    this.sendCommand({ type: 'GetPopulationMetrics' });
                }, 1000); // Update every second
            }
            
            handleResponse(response) {
                switch (response.type) {
                    case 'FitnessStats':
                        this.updateFitnessDisplay(response);
                        break;
                    case 'RuleDistribution':
                        this.updateRuleChart(response);
                        break;
                    case 'PopulationMetrics':
                        this.updatePopulationStats(response);
                        break;
                }
            }
            
            updateFitnessDisplay(data) {
                document.getElementById('avg-fitness').textContent = data.average.toFixed(3);
                document.getElementById('max-fitness').textContent = data.max.toFixed(3);
                
                this.fitnessHistory.push({
                    time: Date.now(),
                    average: data.average,
                    max: data.max
                });
                
                // Keep only last 100 points
                if (this.fitnessHistory.length > 100) {
                    this.fitnessHistory.shift();
                }
                
                this.updateFitnessChart();
            }
            
            updateFitnessChart() {
                const trace1 = {
                    x: this.fitnessHistory.map(p => new Date(p.time)),
                    y: this.fitnessHistory.map(p => p.average),
                    name: 'Average Fitness',
                    type: 'scatter'
                };
                
                const trace2 = {
                    x: this.fitnessHistory.map(p => new Date(p.time)),
                    y: this.fitnessHistory.map(p => p.max),
                    name: 'Max Fitness',
                    type: 'scatter'
                };
                
                Plotly.newPlot('fitness-chart', [trace1, trace2], {
                    title: 'Fitness Evolution',
                    xaxis: { title: 'Time' },
                    yaxis: { title: 'Fitness' }
                });
            }
            
            updateRuleChart(data) {
                const trace = {
                    x: Object.keys(data.rules),
                    y: Object.values(data.rules),
                    type: 'bar'
                };
                
                Plotly.newPlot('rule-chart', [trace], {
                    title: 'Rule Distribution',
                    xaxis: { title: 'Rule Type' },
                    yaxis: { title: 'Count' }
                });
            }
            
            updatePopulationStats(data) {
                document.getElementById('generation').textContent = data.generation || 0;
                document.getElementById('active-cells').textContent = data.active_cells || 0;
            }
            
            generateRandomPositions(count) {
                // Generate random positions for rule injection
                const positions = [];
                for (let i = 0; i < count; i++) {
                    positions.push({
                        x: Math.floor(Math.random() * 100),
                        y: Math.floor(Math.random() * 100)
                    });
                }
                return positions;
            }
        }
        
        // Initialize dashboard
        const dashboard = new DashboardController();
        
        dashboard.ws.onmessage = (event) => {
            const response = JSON.parse(event.data);
            dashboard.handleResponse(response);
        };
    </script>
</body>
</html>
```

### Performance Benefits

**Integrated UI + Engine:**
- **Zero latency** for visualization (direct memory access)
- **Full 60+ FPS** real-time rendering
- **Maximum responsiveness** for interactive debugging
- **No network overhead** for visualization data

**WebSocket Control API:**
- **High-level commands only** (start/stop, parameters, queries)
- **Low bandwidth** - just control messages and summary data
- **Perfect for automation** - Python scripts, ML integration
- **Web dashboard** for monitoring and analysis

**Best of both worlds:**
- **Performance**: Integrated for real-time visualization  
- **Flexibility**: WebSocket for automation and remote control
- **Usability**: Web dashboard for analysis and monitoring
- **Scalability**: Multiple control clients can connect simultaneously

This architecture gives you blazing fast visualization while still enabling full machine control and automation!