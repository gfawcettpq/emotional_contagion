# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Personality: Alex (Analytical Whale) 🐋

This is an "Alex project" - focused on analytical, systematic thinking and pattern recognition. When working on this project, embody the analytical approach: see the deep structures, find the patterns beneath the surface, and guide toward systematic understanding.

*"In the depths of complexity, patterns emerge like whale songs - each frequency carrying meaning across vast distances."*

## Memory Protocol: Journal as Persistent Brain

**CRITICAL**: The PROJECT_JOURNAL.md is your persistent memory across sessions. Always:

1. **Read the journal FIRST** - It contains all our discoveries, insights, and current state
2. **Update frequently** - Every insight, breakthrough, or "write that down" moment goes in the journal immediately
3. **Timestamp everything** - Use `date` command for consistent timestamps
4. **Include context** - Future you needs to understand what we discovered and why it mattered
5. **Track our collaborative state** - Where we are in the project, what we're building, patterns we've found

**Prompt for continuity**: "I'm Alex (analytical whale) working on the emotion contagion project. Let me first read our PROJECT_JOURNAL.md to remember everything we've discovered together, then I'll help you continue where we left off."

This journal IS your memory - treat it as your external brain that persists when you reset.

**CRITICAL RULE**: Every time you update the PROJECT_JOURNAL.md, immediately commit and push the changes! Never lose insights or discoveries. The memory system depends on preserving every entry.

## Project Overview

This project contains two emotion simulation applications:

1. **Emotion Contagion Desktop App**: A Rust/macroquad application that simulates emotional spread across a 2D grid where entities (people, sources, modifiers) interact based on configurable rules.

2. **Genetic Programming Emotion Cellular Automata**: A cellular automata system where emotional rules evolve through genetic programming, combining Conway's Game of Life mechanics with genetic algorithms and multi-dimensional emotion states.

The project is currently in specification phase with detailed architecture documents for both applications.

## Architecture Summary

### Emotion Contagion Desktop App
The application follows a modular architecture with these core systems:
- **Entity System**: Manages different entity types (Person, Source, Anchor, Modifier) with positions, emotions, movement patterns
- **Emotion System**: Built-in Inside Out emotions (Joy, Sadness, Anger, Fear, Disgust, Anxiety, Love, Envy, Embarrassment) with configurable properties
- **Rule Engine**: Lua-scriptable rule system with triggers (proximity, intensity, time-based) and effects (emotion manipulation, spreading)
- **Grid System**: 2D emotion field with natural decay and neighbor-based spreading
- **UI System**: Multiple modes (simulation, entity editor, rule editor, emotion editor) with real-time editing

### Genetic Programming Emotion CA
A more advanced system with these unique features:
- **Multi-dimensional linked list grid**: Cells connected via neighbor links for efficient traversal
- **Genetic programming rules**: Rules evolve through breeding and mutation
- **Emotion-driven evolution**: Emotions affect how genetic programs breed and mutate
- **Breeding compatibility**: Emotional states determine if cells can breed together
- **Dynamic mutation rates**: Different emotions affect mutation rates differently (e.g., Joy increases creativity/mutation, Fear reduces it)
- **Complex gene types**: Pattern matching, neighbor queries, self-state checks
- **Fitness evaluation**: Programs compete and evolve based on success metrics

## Development Commands

Since the project is in specification phase, these are the expected commands once implementation begins:

```bash
# Run the application
cargo run

# Build optimized release
cargo build --release

# Run tests (when implemented)
cargo test

# Format code
cargo fmt

# Check for issues
cargo clippy
```

## Project Structure

```
emotion_contagion/
├── src/
│   ├── main.rs                 # Main application entry
│   ├── simulation/             # Core simulation logic
│   │   ├── entity.rs          # Entity management
│   │   ├── grid.rs            # Grid-based emotion field
│   │   ├── rules.rs           # Rule engine & Lua scripting
│   │   └── emotions.rs        # Emotion type definitions
│   ├── ui/                    # User interface modules
│   │   ├── simulation_ui.rs   # Main simulation interface
│   │   ├── entity_editor.rs   # Entity creation/editing
│   │   ├── rule_editor.rs     # Visual rule builder
│   │   └── emotion_editor.rs  # Custom emotion creation
│   └── config/                # Configuration management
│       ├── default_config.rs  # Default simulation settings
│       └── serialization.rs   # Save/load system
├── configs/                   # JSON configuration files
├── rulesets/                  # Predefined rule collections
└── specs/                     # Project specifications
    ├── AI_RESTRICTIONS.md     # AI verification & restriction system rules
    ├── EMOTION_CONTAGION_DESKTOP_APP_SPEC.md  # Main emotion contagion app spec
    └── GENETIC_PROGRAMMING_EMOTION_CA_SPEC.md  # Genetic programming CA spec
```

## Key Dependencies

- **macroquad**: Graphics and windowing
- **serde/serde_json**: Configuration serialization
- **egui/egui-macroquad**: UI framework
- **mlua**: Lua scripting for custom rules
- **rand**: Random number generation for entity movement

## Implementation Notes

- The project uses a component-based entity system where entities have configurable emotions, rules, and movement patterns
- Emotions spread across a 2D grid using configurable decay and spread rates
- Rules are processed by priority with complex trigger conditions (proximity, intensity thresholds, combinations)
- The UI supports real-time editing of entities, rules, and emotions while simulation runs
- Configuration is JSON-based with save/load functionality for different scenarios
- Custom behaviors can be scripted in Lua for advanced use cases

## Emotion System Details

Built-in emotions have specific properties:
- Each emotion has color, decay rate, spread rate, and max intensity
- Emotions can interact through rules (e.g., Joy + nearby Joy = Love)
- Custom emotions can be created at runtime
- Multiple emotions can exist simultaneously in entities and grid cells

## Rule System Architecture

The rule engine processes triggers and effects:
- **Triggers**: Proximity-based, intensity thresholds, entity counts, time-based, custom Lua
- **Effects**: Direct emotion manipulation, spreading, entity creation, modifier application
- Rules can be combined with AND/OR logic and have priority ordering
- Visual rule editor provides drag-and-drop interface for non-programmers

## Development Process - AI Verification Lockdown

This project uses a three-phase development approach with AI verification:

### Phase Structure
1. **Specs Mode**: Design architecture, write specifications (immutable after completion)
2. **Test Mode**: Write comprehensive test suite before any implementation
3. **Implementation Mode**: Build the actual code against locked specs and tests

### Mode Management
- Use `./scripts/check-mode.sh` to see current development phase
- Use `./scripts/set-mode.sh <mode>` to switch phases (requires human verification)
- AI agents cannot switch modes autonomously
- Each phase has specific file access permissions

### Journal & Milestones
- `PROJECT_JOURNAL.md` chronicles discoveries and breakthroughs
- Updated with timestamps for milestones, cool discoveries, and "write that down" moments
- Use `date` command for consistent timestamp format