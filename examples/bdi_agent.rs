use z_cognition::prelude::*;

fn main() {
    // Create a BDI agent
    let mut agent = BDIAgent::new();

    // Add beliefs
    agent
        .beliefs_mut()
        .add(Belief::fact("location", "warehouse"));
    agent.beliefs_mut().add(Belief::fact("battery_level", "80"));

    // Add desires
    let goal = Goal::achievement("deliver_package").with_priority(0.9);
    agent.add_desire(Desire::new(goal, 0.9));

    // Display agent state
    println!("BDI Agent Created:");
    println!("  Beliefs: {}", agent.beliefs().len());
    println!("  Desires: {}", agent.desires().len());
    println!("  Intentions: {}", agent.intentions().len());

    // Query beliefs
    if let Some(location) = agent.beliefs().get("location") {
        println!("  Current location: {}", location.value());
    }
}
