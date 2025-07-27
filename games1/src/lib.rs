// Emotion Contagion Engine
// A 2D visual simulation of emotion spreading between entities

pub mod simulation;

// Re-export commonly used items
pub use simulation::{
    emotions::*,
    entity::*,
    grid::*,
    colors::*,
};