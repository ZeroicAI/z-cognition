use super::Rule;

/// Result of a rule match with confidence score
#[derive(Debug, Clone)]
pub struct Inference {
    pub rule_name: String,
    pub conclusions: Vec<String>,
    pub confidence: f64,
}

/// Simple reasoning engine using forward chaining
#[derive(Debug)]
pub struct ReasoningEngine {
    rules: Vec<Rule>,
}

impl ReasoningEngine {
    /// Create a new reasoning engine
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Add a rule
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    /// Get all rules
    pub fn rules(&self) -> &[Rule] {
        &self.rules
    }

    /// Perform inference (placeholder - kept for backward compatibility)
    pub fn infer(&self) -> Vec<String> {
        Vec::new()
    }

    /// Match rules against a set of facts (keywords from input)
    /// Returns inferences sorted by confidence (highest first)
    pub fn infer_from_facts(&self, facts: &[String]) -> Vec<Inference> {
        let lower_facts: Vec<String> = facts.iter().map(|f| f.to_lowercase()).collect();

        let mut inferences: Vec<Inference> = self
            .rules
            .iter()
            .filter_map(|rule| {
                let matched = rule
                    .conditions()
                    .iter()
                    .filter(|cond| {
                        let cond_lower = cond.to_lowercase();
                        lower_facts.iter().any(|fact| fact.contains(&cond_lower))
                    })
                    .count();

                if matched == 0 {
                    return None;
                }

                let total = rule.conditions().len().max(1);
                let confidence = matched as f64 / total as f64;

                Some(Inference {
                    rule_name: rule.name().to_string(),
                    conclusions: rule.conclusions().to_vec(),
                    confidence,
                })
            })
            .collect();

        // Sort by confidence descending
        inferences.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

        inferences
    }

    /// Get the best matching inference, if any
    pub fn best_match(&self, facts: &[String]) -> Option<Inference> {
        self.infer_from_facts(facts).into_iter().next()
    }
}

impl Default for ReasoningEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_from_facts_basic() {
        let mut engine = ReasoningEngine::new();
        engine.add_rule(
            Rule::new("bdi_topic")
                .with_condition("bdi")
                .with_condition("belief")
                .with_condition("desire")
                .with_conclusion("topic:bdi"),
        );
        engine.add_rule(
            Rule::new("swarm_topic")
                .with_condition("swarm")
                .with_condition("flock")
                .with_conclusion("topic:swarm"),
        );

        let facts = vec!["what".into(), "is".into(), "bdi".into()];
        let results = engine.infer_from_facts(&facts);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].rule_name, "bdi_topic");
        assert_eq!(results[0].conclusions, vec!["topic:bdi"]);
    }

    #[test]
    fn test_confidence_ranking() {
        let mut engine = ReasoningEngine::new();
        engine.add_rule(
            Rule::new("narrow")
                .with_condition("swarm")
                .with_conclusion("topic:swarm"),
        );
        engine.add_rule(
            Rule::new("broad")
                .with_condition("swarm")
                .with_condition("flock")
                .with_condition("consensus")
                .with_conclusion("topic:swarm_deep"),
        );

        // Only matches "swarm" — narrow rule has 1/1 = 1.0, broad has 1/3 = 0.33
        let facts = vec!["swarm".into()];
        let results = engine.infer_from_facts(&facts);

        assert_eq!(results[0].rule_name, "narrow");
        assert!(results[0].confidence > results[1].confidence);
    }

    #[test]
    fn test_no_match_returns_empty() {
        let mut engine = ReasoningEngine::new();
        engine.add_rule(
            Rule::new("bdi_topic")
                .with_condition("bdi")
                .with_conclusion("topic:bdi"),
        );

        let facts = vec!["hello".into(), "world".into()];
        let results = engine.infer_from_facts(&facts);
        assert!(results.is_empty());
    }

    #[test]
    fn test_best_match() {
        let mut engine = ReasoningEngine::new();
        engine.add_rule(
            Rule::new("patterns")
                .with_condition("pattern")
                .with_conclusion("topic:patterns"),
        );

        let facts = vec!["what".into(), "pattern".into(), "support".into()];
        let best = engine.best_match(&facts);

        assert!(best.is_some());
        assert_eq!(best.unwrap().rule_name, "patterns");
    }
}
