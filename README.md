# zeroicai-cognition

[![Crates.io](https://img.shields.io/crates/v/zeroicai-cognition.svg)](https://crates.io/crates/zeroicai-cognition)
[![Documentation](https://docs.rs/zeroicai-cognition/badge.svg)](https://docs.rs/zeroicai-cognition)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

**Reasoning, planning, decision-making, and cognitive architectures for intelligent agents.**

`zeroicai-cognition` provides the "brain" of autonomous agents, implementing cognitive architectures, reasoning engines, planning algorithms and decision-making frameworks. It enables agents to think, reason, plan and make intelligent decisions based on their beliefs, desires and intentions.

---

## Purpose

This crate provides:

- **BDI Architecture**: Belief-Desire-Intention cognitive model
- **Planning**: Goal-oriented action planning (STRIPS, HTN, PDDL)
- **Reasoning**: Logical inference and rule-based reasoning
- **Decision-Making**: Utility-based and probabilistic decision frameworks
- **Learning**: Adaptation and knowledge acquisition

---

## Core Concepts

### BDI Architecture

The Belief-Desire-Intention model is a mature cognitive architecture for deliberative agents:
```rust
use zeroicai_cognition::{BeliefBase, DesireSet, IntentionStack, BDIAgent};

// Beliefs - What the agent knows about the world
let mut beliefs = BeliefBase::new();
beliefs.add_belief("temperature", 25.0);
beliefs.add_belief("market_open", true);

// Desires - What the agent wants to achieve
let mut desires = DesireSet::new();
desires.add_desire(Desire::new("maximize_profit", 1.0));
desires.add_desire(Desire::new("minimize_risk", 0.8));

// Intentions - What the agent has committed to do
let mut intentions = IntentionStack::new();
intentions.push(Intention::new("buy_stock", plan));

// BDI Agent
let agent = BDIAgent::new(beliefs, desires, intentions);
```

### Beliefs

Beliefs represent the agent's knowledge about the world:
```rust
use zeroicai_cognition::{Belief, BeliefBase, BeliefRevision};

let mut belief_base = BeliefBase::new();

// Add beliefs
belief_base.add(Belief::fact("is_raining", true));
belief_base.add(Belief::rule("wet_ground", "is_raining"));

// Query beliefs
if belief_base.query("is_raining")? {
    println!("It's raining!");
}

// Belief revision (handle contradictions)
belief_base.revise(Belief::fact("is_raining", false))?;
```

### Desires (Goals)

Desires represent the agent's motivational states:
```rust
use zeroicai_cognition::{Desire, Goal, GoalType};

// Achievement goal
let goal = Goal::new(
    "reach_destination",
    GoalType::Achievement,
    vec!["at_location(home)"]
);

// Maintenance goal
let maintenance = Goal::new(
    "stay_charged",
    GoalType::Maintenance,
    vec!["battery_level > 20"]
);

// Goal priority
let desire = Desire::new(goal, 0.9); // Priority: 0.9
```

### Intentions (Plans)

Intentions are commitments to execute plans:
```rust
use zeroicai_cognition::{Intention, Plan, Action};

// Define a plan
let plan = Plan::new("travel_home")
    .add_step(Action::new("start_car"))
    .add_step(Action::new("drive_to", vec!["home"]))
    .add_step(Action::new("park_car"));

// Create intention
let intention = Intention::new("go_home", plan);

// Execute intention
intention.execute(&mut context).await?;
```

### Planning

Goal-oriented action planning:
```rust
use zeroicai_cognition::{Planner, State, Goal, Action};

// Define initial state
let initial_state = State::new()
    .set("at", "office")
    .set("has_keys", true);

// Define goal
let goal = State::new()
    .set("at", "home");

// Define available actions
let actions = vec![
    Action::new("drive_home")
        .precondition("at", "office")
        .precondition("has_keys", true)
        .effect("at", "home"),
];

// Create planner
let planner = Planner::new(actions);

// Generate plan
let plan = planner.plan(&initial_state, &goal)?;
```

### Decision-Making

Utility-based and probabilistic decision frameworks:
```rust
use zeroicai_cognition::{DecisionMaker, Decision, Utility};

let decision_maker = DecisionMaker::new();

// Define options with utilities
let options = vec![
    Decision::new("invest_stocks").utility(0.7).risk(0.5),
    Decision::new("invest_bonds").utility(0.5).risk(0.2),
    Decision::new("hold_cash").utility(0.3).risk(0.0),
];

// Make decision (maximize expected utility)
let choice = decision_maker.decide(&options, &beliefs)?;
```

### Reasoning

Logical inference and rule-based reasoning:
```rust
use zeroicai_cognition::{ReasoningEngine, Rule, Fact};

let mut engine = ReasoningEngine::new();

// Add facts
engine.add_fact(Fact::new("mortal", vec!["socrates"]));
engine.add_fact(Fact::new("human", vec!["socrates"]));

// Add rules
engine.add_rule(Rule::new(
    "mortality_rule",
    vec!["human(X)"],
    vec!["mortal(X)"]
));

// Infer new knowledge
let inferred = engine.infer()?;
```

---

## What's Included

### BDI Components

- `BeliefBase` - Knowledge representation and belief management
- `DesireSet` - Goal and desire management
- `IntentionStack` - Plan execution and commitment
- `BDIAgent` - Complete BDI agent implementation
- `BeliefRevision` - Handle contradictory beliefs

### Planning

- `Planner` - STRIPS-style planning
- `HTNPlanner` - Hierarchical Task Network planning
- `Plan` - Ordered sequence of actions
- `Action` - Atomic actions with preconditions and effects
- `State` - World state representation

### Decision-Making

- `DecisionMaker` - Utility-based decision framework
- `UtilityFunction` - Preference modeling
- `ExpectedUtility` - Decision under uncertainty
- `MultiCriteria` - Multi-objective decision-making

### Reasoning

- `ReasoningEngine` - Forward and backward chaining
- `Rule` - Logical inference rules
- `Fact` - Ground facts
- `Query` - Knowledge base queries
- `Unification` - Pattern matching

### Learning

- `ReinforcementLearner` - Q-learning and policy gradient
- `KnowledgeAcquisition` - Learn from experience
- `AdaptivePlanner` - Learn planning heuristics

---

## Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
zeroicai-cognition = "0.1.0"
zeroicai-core = "0.1.0"
```

### Complete BDI Agent Example
```rust
use zeroicai_cognition::{
    BDIAgent, BeliefBase, DesireSet, IntentionStack,
    Belief, Desire, Goal, Intention, Plan, Action
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize beliefs
    let mut beliefs = BeliefBase::new();
    beliefs.add(Belief::fact("battery_level", 80));
    beliefs.add(Belief::fact("location", "warehouse"));
    beliefs.add(Belief::fact("has_package", false));

    // Initialize desires
    let mut desires = DesireSet::new();
    desires.add(Desire::new(
        Goal::achievement("deliver_package"),
        0.9
    ));

    // Initialize intentions
    let intentions = IntentionStack::new();

    // Create BDI agent
    let mut agent = BDIAgent::new(beliefs, desires, intentions);

    // BDI reasoning cycle
    loop {
        // 1. Belief revision (update beliefs from perception)
        agent.update_beliefs(percepts).await?;

        // 2. Option generation (determine possible goals)
        let options = agent.generate_options().await?;

        // 3. Deliberation (select goals to pursue)
        let selected_goals = agent.deliberate(options).await?;

        // 4. Means-ends reasoning (plan how to achieve goals)
        let plans = agent.plan(selected_goals).await?;

        // 5. Intention adoption (commit to plans)
        agent.adopt_intentions(plans).await?;

        // 6. Execution (execute current intentions)
        agent.execute().await?;

        // Check if goals achieved
        if agent.goals_satisfied() {
            break;
        }
    }

    Ok(())
}
```

### Planning Example
```rust
use zeroicai_cognition::{Planner, State, Action};

async fn plan_delivery() -> Result<(), Box<dyn std::error::Error>> {
    // Initial state
    let initial = State::new()
        .set("at", "warehouse")
        .set("has_package", false)
        .set("package_at", "warehouse");

    // Goal state
    let goal = State::new()
        .set("at", "customer")
        .set("has_package", false)
        .set("package_at", "customer");

    // Available actions
    let actions = vec![
        Action::new("pick_up_package")
            .precondition("at", "warehouse")
            .precondition("has_package", false)
            .precondition("package_at", "warehouse")
            .effect("has_package", true),
        
        Action::new("drive_to_customer")
            .precondition("at", "warehouse")
            .precondition("has_package", true)
            .effect("at", "customer"),
        
        Action::new("deliver_package")
            .precondition("at", "customer")
            .precondition("has_package", true)
            .effect("has_package", false)
            .effect("package_at", "customer"),
    ];

    // Plan
    let planner = Planner::new(actions);
    let plan = planner.plan(&initial, &goal)?;

    println!("Generated plan:");
    for action in plan.actions() {
        println!("  - {}", action.name());
    }

    Ok(())
}
```

### Reasoning Example
```rust
use zeroicai_cognition::{ReasoningEngine, Rule, Fact};

fn reasoning_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = ReasoningEngine::new();

    // Add facts
    engine.add_fact(Fact::new("temperature", vec!["30"]));
    engine.add_fact(Fact::new("humidity", vec!["high"]));

    // Add rules
    engine.add_rule(Rule::new(
        "hot_weather",
        vec!["temperature(X)", "X > 25"],
        vec!["hot"]
    ));

    engine.add_rule(Rule::new(
        "uncomfortable",
        vec!["hot", "humidity(high)"],
        vec!["uncomfortable"]
    ));

    // Infer
    engine.infer()?;

    // Query
    if engine.query("uncomfortable")? {
        println!("It's uncomfortable outside!");
    }

    Ok(())
}
```

---

## Architecture

### Cognitive Cycle

The BDI reasoning cycle:

1. **Perceive** → Update beliefs from environment
2. **Deliberate** → Select goals to pursue
3. **Plan** → Generate plans to achieve goals
4. **Execute** → Perform actions from current intention
5. **Repeat** → Continue until goals satisfied

### Planning Algorithms

- **STRIPS** - Classical planning with preconditions and effects
- **HTN** - Hierarchical decomposition of tasks
- **Partial-Order Planning** - Flexible action ordering
- **Reactive Planning** - Interleaving planning and execution

### Decision Theory

- **Expected Utility Theory** - Rational decision-making
- **Multi-Attribute Utility** - Handle multiple objectives
- **Prospect Theory** - Human-like decision biases

---

## Related Crates

- **[zeroicai-core](../zeroicai-core)** - Agent primitives and traits
- **[zeroicai-messaging](../zeroicai-messaging)** - Agent communication
- **[zeroicai-patterns](../zeroicai-patterns)** - Multi-agent coordination
- **[zeroicai-runtime](../zeroicai-runtime)** - Agent execution engine

---

## Documentation

Full API documentation is available on [docs.rs](https://docs.rs/zeroicai-cognition).

For guides and tutorials, see [zeroicai-docs](https://github.com/zeroicai/zeroicai-docs).

---

## References

This crate is inspired by:

- **BDI Architecture** - Rao & Georgeff (1991)
- **STRIPS Planning** - Fikes & Nilsson (1971)
- **HTN Planning** - Erol, Hendler & Nau (1994)
- **Belief Revision** - AGM Theory (Alchourrón, Gärdenfors & Makinson)
- **Decision Theory** - Von Neumann & Morgenstern

---

## Contributing

Contributions are welcome! Please see the [contributing guidelines](../../CONTRIBUTING.md).

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

## Status

**Active Development** - This crate is under active development. APIs may change before 1.0 release.

---

*Part of the [ZeroicAI](https://github.com/zeroicai) ecosystem for agent-oriented programming in Rust.*
