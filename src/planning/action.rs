use serde::{Deserialize, Serialize};
use super::State;

/// An action with STRIPS-style preconditions and effects
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    name: String,
    parameters: Vec<String>,
    preconditions: Vec<(String, String)>,
    effects: Vec<(String, String)>,
}

impl Action {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parameters: Vec::new(),
            preconditions: Vec::new(),
            effects: Vec::new(),
        }
    }

    pub fn with_parameter(mut self, param: impl Into<String>) -> Self {
        self.parameters.push(param.into());
        self
    }

    /// Add a precondition: the state variable `key` must equal `value`
    pub fn with_precondition(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.preconditions.push((key.into(), value.into()));
        self
    }

    /// Add an effect: set state variable `key` to `value` after execution
    pub fn with_effect(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.effects.push((key.into(), value.into()));
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parameters(&self) -> &[String] {
        &self.parameters
    }

    pub fn preconditions(&self) -> &[(String, String)] {
        &self.preconditions
    }

    pub fn effects(&self) -> &[(String, String)] {
        &self.effects
    }

    /// Returns true if all preconditions hold in the given state
    pub fn is_applicable(&self, state: &State) -> bool {
        self.preconditions.iter().all(|(k, v)| state.matches(k, v))
    }

    /// Apply this action's effects to a state, returning the resulting state
    pub fn apply(&self, state: &State) -> State {
        let mut next = state.clone();
        for (k, v) in &self.effects {
            next = next.set(k.clone(), v.clone());
        }
        next
    }
}
