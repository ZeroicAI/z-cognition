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

    /// Create a simple utility function with default evaluator
    pub fn simple(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            evaluator: Box::new(|_| 0.5),
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

