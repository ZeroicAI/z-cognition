use super::{BeliefBase, Desire, IntentionStack};

/// BDI Agent structure
#[derive(Debug, Clone)]
pub struct BDIAgent {
    beliefs: BeliefBase,
    desires: Vec<Desire>,
    intentions: IntentionStack,
}

impl BDIAgent {
    /// Create new BDI agent
    pub fn new() -> Self {
        Self {
            beliefs: BeliefBase::new(),
            desires: Vec::new(),
            intentions: IntentionStack::new(),
        }
    }

    /// Create with components
    pub fn with_components(
        beliefs: BeliefBase,
        desires: Vec<Desire>,
        intentions: IntentionStack,
    ) -> Self {
        Self {
            beliefs,
            desires,
            intentions,
        }
    }

    /// Get beliefs
    pub fn beliefs(&self) -> &BeliefBase {
        &self.beliefs
    }

    /// Get mutable beliefs
    pub fn beliefs_mut(&mut self) -> &mut BeliefBase {
        &mut self.beliefs
    }

    /// Get desires
    pub fn desires(&self) -> &[Desire] {
        &self.desires
    }

    /// Get mutable desires
    pub fn desires_mut(&mut self) -> &mut Vec<Desire> {
        &mut self.desires
    }

    /// Get intentions
    pub fn intentions(&self) -> &IntentionStack {
        &self.intentions
    }

    /// Get mutable intentions
    pub fn intentions_mut(&mut self) -> &mut IntentionStack {
        &mut self.intentions
    }

    /// Add a desire
    pub fn add_desire(&mut self, desire: Desire) {
        self.desires.push(desire);
    }
}

impl Default for BDIAgent {
    fn default() -> Self {
        Self::new()
    }
}
