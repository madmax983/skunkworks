use bio_transit::{Agent, Settings, Simulation, TrailMap};
use macroquad::prelude::*;

#[test]
fn test_agent_move() {
    // Initial state
    let home = vec2(50.0, 50.0);
    let work = vec2(150.0, 150.0);
    let mut agent = Agent::new(vec2(100.0, 100.0), 0.0, home, work);
    let map = TrailMap::new(200, 200);
    let settings = Settings::default();

    // Check initial position
    assert_eq!(agent.pos, vec2(100.0, 100.0));

    // Update
    agent.update(&map, &settings);

    // Verify movement
    assert_ne!(agent.pos, vec2(100.0, 100.0), "Agent did not move!");
}

#[test]
fn test_trail_deposit() {
    let mut map = TrailMap::new(10, 10);
    let settings = Settings::default();

    // Deposit
    map.deposit(5, 5, 1.0);
    assert!(map.grid[5 * 10 + 5] > 0.0);

    // Diffuse
    map.diffuse_and_decay(&settings);

    // Verify diffusion
    assert!(map.grid[5 * 10 + 6] > 0.0, "Diffusion did not spread!");
}

#[test]
fn test_simulation_init() {
    let sim = Simulation::new(200, 200, 10);
    assert_eq!(sim.agents.len(), 10);
    assert_eq!(sim.cities.len(), 5);
}
