//! Reasoning, planning, BDI architecture, and decision-making for intelligent agents.

//#![warn(missing_docs)]
#![allow(missing_docs)]

pub mod bdi;
pub mod decision;
pub mod error;
pub mod planning;
pub mod prelude;
pub mod reasoning;

// Re-exports
pub use bdi::{BDIAgent, Belief, BeliefBase, Desire, Goal, Intention, IntentionStack};
pub use error::CognitionError;
pub use planning::{Action, Plan, Planner, State};
pub use reasoning::{Inference, ReasoningEngine, Rule};
pub use decision::UtilityFunction;