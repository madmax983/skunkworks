# The Flock's Mind 🕊️

Flocking is the art of simulating complex group behavior from simple individual rules. This crate implements Craig Reynolds' "Boids" algorithm.

## The Three Laws

1.  **Separation ("Personal Space")**: Steer to avoid crowding local flockmates.
    *   *Too close? Back off.*
2.  **Alignment ("Peer Pressure")**: Steer towards the average heading of local flockmates.
    *   *Everyone going left? I'll go left too.*
3.  **Cohesion ("Group Hug")**: Steer to move toward the average position of local flockmates.
    *   *Don't get left behind.*

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
flocking = { path = "../flocking" }
locus = { path = "../locus" }
```

## The Minimal Simulation

If you are building a simulation where agents move through space in a coordinated manner, `flocking` calculates the steering forces for you.

```rust
use flocking::{compute_force, FlockingParams};
use locus::Vec2;

fn main() {
    // 1. Setup the flock
    let mut positions = vec![Vec2::new(0.0, 0.0), Vec2::new(5.0, 5.0)];
    let mut velocities = vec![Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)];

    // 2. Define the rules (the "DNA" of the flock)
    let params = FlockingParams {
        view_radius: 50.0,
        separation_radius: 10.0,
        max_speed: 2.0,
        max_force: 0.1,
        separation_weight: 1.5, // Strong desire for personal space
        alignment_weight: 1.0,  // Moderate desire to align
        cohesion_weight: 1.0,   // Moderate desire to stay together
    };

    // 3. The Loop (Simulate one frame)
    // Note: In a real sim, you'd calculate ALL forces before applying them to avoid order bias.
    let forces: Vec<Vec2> = (0..positions.len())
        .map(|i| compute_force(&positions, &velocities, i, &params))
        .collect();

    for (i, force) in forces.iter().enumerate() {
        velocities[i] += *force; // Apply steering
        velocities[i] = velocities[i].limit(params.max_speed); // Cap speed
        positions[i] += velocities[i]; // Move

        println!("Agent {} moved to {:?}", i, positions[i]);
    }
}
```
