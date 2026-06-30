use super::{BeliefBase, Desire, Goal, Intention, IntentionStack};
use crate::planning::{Planner, State};

/// BDI Agent — Beliefs, Desires, Intentions
#[derive(Debug, Clone)]
pub struct BDIAgent {
    beliefs: BeliefBase,
    desires: Vec<Desire>,
    intentions: IntentionStack,
}

impl BDIAgent {
    pub fn new() -> Self {
        Self {
            beliefs: BeliefBase::new(),
            desires: Vec::new(),
            intentions: IntentionStack::new(),
        }
    }

    pub fn with_components(
        beliefs: BeliefBase,
        desires: Vec<Desire>,
        intentions: IntentionStack,
    ) -> Self {
        Self { beliefs, desires, intentions }
    }

    pub fn beliefs(&self) -> &BeliefBase { &self.beliefs }
    pub fn beliefs_mut(&mut self) -> &mut BeliefBase { &mut self.beliefs }
    pub fn desires(&self) -> &[Desire] { &self.desires }
    pub fn desires_mut(&mut self) -> &mut Vec<Desire> { &mut self.desires }
    pub fn intentions(&self) -> &IntentionStack { &self.intentions }
    pub fn intentions_mut(&mut self) -> &mut IntentionStack { &mut self.intentions }

    pub fn add_desire(&mut self, desire: Desire) {
        self.desires.push(desire);
    }

    /// BDI deliberation cycle:
    /// 1. Select the highest-priority desire.
    /// 2. Convert its conditions to a goal `State` (each condition = key with value "true").
    /// 3. Use the planner to find a plan from `current_state` to the goal state.
    /// 4. Commit to the plan by pushing an `Intention` onto the stack.
    ///
    /// Returns a reference to the new intention, or `None` if no plan was found.
    pub fn deliberate<'a>(
        &'a mut self,
        planner: &Planner,
        current_state: &State,
    ) -> Option<&'a Intention> {
        self.intentions.remove_completed();

        // Pick highest-priority desire
        let best = self
            .desires
            .iter()
            .max_by(|a, b| a.priority().partial_cmp(&b.priority()).unwrap())?;

        let goal: Goal = best.goal().clone();

        // Build a goal State: each condition string becomes a variable key set to "true"
        let mut goal_state = State::new();
        for cond in goal.conditions() {
            goal_state = goal_state.set(cond.clone(), "true");
        }

        match planner.plan(current_state, &goal_state) {
            Ok(plan) => {
                self.intentions.push(Intention::new(goal, plan));
                self.intentions.current()
            }
            Err(_) => None,
        }
    }
}

impl Default for BDIAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planning::Action;
    use super::super::GoalType;

    #[test]
    fn test_deliberate_finds_plan() {
        let mut agent = BDIAgent::new();

        let goal = Goal::new("light_on", GoalType::Achievement)
            .with_condition("light");

        agent.add_desire(Desire::new(goal, 0.9));

        let action = Action::new("turn_on")
            .with_precondition("light", "off")
            .with_effect("light", "true");

        let planner = Planner::with_actions(vec![action]);
        let current = State::new().set("light", "off");

        let intention = agent.deliberate(&planner, &current);
        assert!(intention.is_some());
        assert_eq!(intention.unwrap().plan().len(), 1);
    }

    #[test]
    fn test_deliberate_no_plan_returns_none() {
        let mut agent = BDIAgent::new();
        let goal = Goal::new("impossible", GoalType::Achievement)
            .with_condition("unreachable");

        agent.add_desire(Desire::new(goal, 1.0));

        let planner = Planner::new(); // no actions
        let current = State::new();

        assert!(agent.deliberate(&planner, &current).is_none());
    }
}
