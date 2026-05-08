use z_cognition::prelude::*;

#[test]
fn create_belief() {
    let belief = Belief::new("temperature", "25");
    assert_eq!(belief.key(), "temperature");
    assert_eq!(belief.value(), "25");
    assert_eq!(belief.certainty(), 1.0);
}

#[test]
fn belief_base_operations() {
    let mut base = BeliefBase::new();

    base.add(Belief::fact("x", "10"));
    assert!(base.contains("x"));
    assert_eq!(base.len(), 1);

    let belief = base.get("x").unwrap();
    assert_eq!(belief.value(), "10");

    base.remove("x");
    assert!(!base.contains("x"));
    assert!(base.is_empty());
}

#[test]
fn create_goal() {
    let goal = Goal::achievement("reach_destination")
        .with_condition("at_location(home)")
        .with_priority(0.9);

    assert_eq!(goal.name(), "reach_destination");
    assert_eq!(goal.priority(), 0.9);
    assert_eq!(goal.conditions().len(), 1);
}

#[test]
fn create_desire() {
    let goal = Goal::achievement("test_goal");
    let desire = Desire::new(goal, 0.8);

    assert_eq!(desire.priority(), 0.8);
    assert_eq!(desire.goal().name(), "test_goal");
}

#[test]
fn intention_stack() {
    let mut stack = IntentionStack::new();
    assert!(stack.is_empty());

    let goal = Goal::achievement("test");
    let plan = Plan::new("test_plan");
    let intention = Intention::new(goal, plan);

    stack.push(intention);
    assert_eq!(stack.len(), 1);
    assert!(stack.current().is_some());
}

#[test]
fn bdi_agent_creation() {
    let agent = BDIAgent::new();

    assert!(agent.beliefs().is_empty());
    assert!(agent.desires().is_empty());
    assert!(agent.intentions().is_empty());
}
