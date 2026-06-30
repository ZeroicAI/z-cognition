use z_cognition::prelude::*;

#[test]
fn create_action() {
    let action = Action::new("move").with_parameter("destination");

    assert_eq!(action.name(), "move");
    assert_eq!(action.parameters().len(), 1);
}

#[test]
fn create_state() {
    let state = State::new().set("location", "home").set("has_key", "true");

    assert!(state.matches("location", "home"));
    assert!(state.matches("has_key", "true"));
}

#[test]
fn create_plan() {
    let plan = Plan::new("test_plan")
        .add_action(Action::new("step1"))
        .add_action(Action::new("step2"));

    assert_eq!(plan.name(), "test_plan");
    assert_eq!(plan.len(), 2);
    assert!(!plan.is_empty());
}

#[test]
fn planner_creation() {
    let planner = Planner::new();
    assert_eq!(planner.actions().len(), 0);

    // Trivially satisfied goal (initial already meets goal) succeeds with no actions
    let state = State::new().set("at", "A");
    assert!(planner.plan(&state, &state.clone()).is_ok());

    // Unsatisfied goal with no available actions returns an error
    let initial = State::new().set("at", "A");
    let goal = State::new().set("at", "B");
    assert!(planner.plan(&initial, &goal).is_err());
}
