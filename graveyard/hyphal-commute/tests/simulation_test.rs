use hyphal_commute::{Agent, World};

#[test]
fn test_agent_movement_and_deposition() {
    let width = 100;
    let height = 100;
    let mut world = World::new(width, height, 1);

    // Force agent to center facing right
    world.agents[0] = Agent::new(50.0, 50.0, 0.0); // Angle 0 is usually Right

    let initial_x = world.agents[0].x;

    // Run one tick
    world.tick();

    // Check movement
    let new_x = world.agents[0].x;
    assert!(
        new_x > initial_x,
        "Agent should move forward. Old: {}, New: {}",
        initial_x,
        new_x
    );

    // Check deposition at previous location (approximate)
    let grid_val = world.grid.get(50, 50);
    assert!(
        grid_val > 0.0,
        "Agent should deposit pheromone at its location. Value: {}",
        grid_val
    );
}

#[test]
fn test_grid_decay() {
    let width = 10;
    let height = 10;
    let mut world = World::new(width, height, 0);

    // Manually set a pixel
    world.grid.cells[0] = 1.0;

    world.tick();

    let new_val = world.grid.cells[0];
    assert!(
        new_val < 1.0,
        "Grid value should decay. Old: 1.0, New: {}",
        new_val
    );
    assert!(
        new_val > 0.0,
        "Grid value should not disappear instantly. New: {}",
        new_val
    );
}
