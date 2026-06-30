/// Utility function for decision-making

pub struct UtilityFunction {
    name: String,
    evaluator: Box<dyn Fn(&[String]) -> f64 + Send + Sync>,
}

impl UtilityFunction {
    /// Create a new utility function with a custom evaluator
    pub fn new(name: impl Into<String>, evaluator: impl Fn(&[String]) -> f64 + Send + Sync + 'static) -> Self {
        Self {
            name: name.into(),
            evaluator: Box::new(evaluator),
        }
    }

    /// Create a utility function that scores state as the proportion of
    /// non-empty, non-false entries — a sensible default for binary goal states.
    pub fn simple(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            evaluator: Box::new(|state| {
                if state.is_empty() {
                    return 0.0;
                }
                let positive = state
                    .iter()
                    .filter(|s| !s.is_empty() && s.as_str() != "false" && s.as_str() != "0")
                    .count();
                positive as f64 / state.len() as f64
            }),
        }
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Evaluate utility
    pub fn evaluate(&self, state: &[String]) -> f64 {
        (self.evaluator)(state)
    }
}

impl std::fmt::Debug for UtilityFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UtilityFunction")
            .field("name", &self.name)
            .field("evaluator", &"<fn>")
            .finish()
    }
}

