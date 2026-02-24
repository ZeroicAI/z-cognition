//! Reasoning module for logical inference
pub mod engine;
pub mod rule;
pub use engine::{Inference, ReasoningEngine};
pub use rule::Rule;
