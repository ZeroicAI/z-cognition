use super::{Action, Plan, State};
use crate::CognitionError;
use std::collections::{HashSet, VecDeque};

/// Forward-search (BFS) STRIPS planner
#[derive(Debug)]
pub struct Planner {
    actions: Vec<Action>,
    max_depth: usize,
}

impl Planner {
    pub fn new() -> Self {
        Self { actions: Vec::new(), max_depth: 20 }
    }

    pub fn with_actions(actions: Vec<Action>) -> Self {
        Self { actions, max_depth: 20 }
    }

    /// Override the default search depth limit (default: 20)
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub fn actions(&self) -> &[Action] {
        &self.actions
    }

    /// Find a plan from `initial` to `goal` using BFS over the state space.
    ///
    /// Returns the shortest sequence of actions that transforms `initial`
    /// into a state satisfying every variable in `goal`, or an error if no
    /// such plan exists within `max_depth` steps.
    pub fn plan(&self, initial: &State, goal: &State) -> Result<Plan, CognitionError> {
        if initial.satisfies(goal) {
            return Ok(Plan::new("trivial"));
        }

        if self.actions.is_empty() {
            return Err(CognitionError::PlanningFailed(
                "No actions available".into(),
            ));
        }

        // BFS queue: (current state, actions taken so far)
        let mut queue: VecDeque<(State, Vec<Action>)> = VecDeque::new();
        queue.push_back((initial.clone(), Vec::new()));

        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(initial.canonical_key());

        while let Some((state, taken)) = queue.pop_front() {
            if taken.len() >= self.max_depth {
                continue;
            }

            for action in &self.actions {
                if !action.is_applicable(&state) {
                    continue;
                }

                let next = action.apply(&state);
                let key = next.canonical_key();

                if visited.contains(&key) {
                    continue;
                }
                visited.insert(key);

                let mut next_taken = taken.clone();
                next_taken.push(action.clone());

                if next.satisfies(goal) {
                    let mut plan = Plan::new(format!("plan_{}_steps", next_taken.len()));
                    for a in next_taken {
                        plan = plan.add_action(a);
                    }
                    return Ok(plan);
                }

                queue.push_back((next, next_taken));
            }
        }

        Err(CognitionError::GoalNotAchievable(format!(
            "No plan found within {} steps",
            self.max_depth
        )))
    }
}

impl Default for Planner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trivial_plan_already_at_goal() {
        let planner = Planner::new();
        let state = State::new().set("x", "1");
        let goal = State::new().set("x", "1");
        let plan = planner.plan(&state, &goal).unwrap();
        assert!(plan.is_empty());
        assert_eq!(plan.name(), "trivial");
    }

    #[test]
    fn test_single_step_plan() {
        let action = Action::new("turn_on")
            .with_precondition("light", "off")
            .with_effect("light", "on");

        let planner = Planner::with_actions(vec![action]);
        let initial = State::new().set("light", "off");
        let goal = State::new().set("light", "on");

        let plan = planner.plan(&initial, &goal).unwrap();
        assert_eq!(plan.len(), 1);
        assert_eq!(plan.actions()[0].name(), "turn_on");
    }

    #[test]
    fn test_multi_step_plan() {
        let pick_up = Action::new("pick_up")
            .with_precondition("holding", "nothing")
            .with_precondition("block_on_table", "true")
            .with_effect("holding", "block")
            .with_effect("block_on_table", "false");

        let stack = Action::new("stack")
            .with_precondition("holding", "block")
            .with_effect("holding", "nothing")
            .with_effect("block_stacked", "true");

        let planner = Planner::with_actions(vec![pick_up, stack]);

        let initial = State::new()
            .set("holding", "nothing")
            .set("block_on_table", "true")
            .set("block_stacked", "false");

        let goal = State::new().set("block_stacked", "true");

        let plan = planner.plan(&initial, &goal).unwrap();
        assert_eq!(plan.len(), 2);
        assert_eq!(plan.actions()[0].name(), "pick_up");
        assert_eq!(plan.actions()[1].name(), "stack");
    }

    #[test]
    fn test_no_plan_returns_error() {
        let action = Action::new("useless")
            .with_precondition("x", "1")
            .with_effect("x", "2");

        let planner = Planner::with_actions(vec![action]);
        let initial = State::new().set("x", "99");
        let goal = State::new().set("x", "1");

        assert!(planner.plan(&initial, &goal).is_err());
    }

    #[test]
    fn test_no_actions_returns_error() {
        let planner = Planner::new();
        let initial = State::new().set("x", "0");
        let goal = State::new().set("x", "1");
        assert!(planner.plan(&initial, &goal).is_err());
    }
}
