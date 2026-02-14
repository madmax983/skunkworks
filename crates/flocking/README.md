# Flocking 🕊️

A lightweight implementation of Reynolds' Flocking algorithm (Boids) for simulating emergent group behavior.

This crate provides the core physics and steering behaviors—Separation, Alignment, and Cohesion—allowing you to easily integrate flocking mechanics into your game or simulation.

## Features

- **Core Behaviors**:
  - **Separation**: Steer to avoid crowding local flockmates.
  - **Alignment**: Steer towards the average heading of local flockmates.
  - **Cohesion**: Steer to move toward the average position of local flockmates.
- **Physics Integration**: Simple position/velocity/acceleration model included.
- **Tunable**: All parameters (weights, radii, speeds) are configurable via `FlockingParams`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
flocking = { path = "crates/flocking" }
# or if using workspace:
flocking = { workspace = true }
```

## Usage

### The Hero's Journey: A Simple Flock

Here is a minimal example of creating a flock, computing forces, and updating positions.

```rust
use flocking::{PhysicsState, FlockingParams, compute_force};
use locus::Vec2;

fn main() {
    // 1. Setup Parameters
    let params = FlockingParams {
        view_radius: 50.0,       // Agents see others within 50 units
        separation_radius: 20.0, // Agents avoid others within 20 units
        max_speed: 4.0,          // Maximum movement speed per tick
        max_force: 0.1,          // Maximum steering force (turn rate)
        separation_weight: 1.5,  // Priority of avoiding collisions
        alignment_weight: 1.0,   // Priority of matching velocity
        cohesion_weight: 1.0,    // Priority of staying together
    };

    // 2. Create Agents
    let mut agents = vec![
        PhysicsState::new(0.0, 0.0),
        PhysicsState::new(10.0, 0.0), // Close neighbor
        PhysicsState::new(40.0, 0.0), // Distant neighbor
    ];

    // Give them some initial velocity
    agents[0].velocity = Vec2::new(1.0, 0.0);
    agents[1].velocity = Vec2::new(0.5, 0.5);
    agents[2].velocity = Vec2::new(-1.0, 0.0);

    // 3. Simulation Loop (Single Step)

    // Calculate forces for all agents based on current state
    // Note: We collect forces first to avoid modifying state while reading it
    let forces: Vec<Vec2> = (0..agents.len())
        .map(|i| compute_force(&agents, i, &params))
        .collect();

    // Apply forces and update physics
    for (agent, force) in agents.iter_mut().zip(forces) {
        agent.apply_force(force);
        agent.update(params.max_speed);
    }

    println!("Agent 0 new position: {:?}", agents[0].position);
}
```

## The Fine Print

### Performance
The `compute_force` function iterates over all other agents to find neighbors, resulting in an **O(N^2)** complexity for a naive implementation.
- **For small flocks (< 500 agents)**: This implementation is fast enough.
- **For large flocks**: Consider using a spatial partition (like a QuadTree or Grid) to limit the search to nearby neighbors. This crate does not provide spatial partitioning out of the box.

### Parameter Tuning
- **`max_force`**: Controls "agility". Lower values make agents turn slowly (like ships); higher values make them turn instantly (like flies).
- **`separation_weight`**: Should typically be higher than cohesion/alignment to prevent agents from collapsing into a single point.
