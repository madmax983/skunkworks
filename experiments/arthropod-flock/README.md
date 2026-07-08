# Arthropod Flock 🐜🕊️

**Concept:** Interactive Swarm Intelligence.

This hybrid visualizer explores what happens when we cross the immediate-mode UI library of `arthropod` with the swarm simulation rules of `flocking`. Instead of a passive boids simulation, the user has direct control over the "DNA" or fundamental rules (cohesion, alignment, separation) governing the swarm.

## Lineage
- **Parent A (crates/arthropod):** Provides the interactive immediate mode graphical UI buttons and color logic.
- **Parent B (crates/flocking):** Provides the robust, optimized Boids algorithm (`compute_force`), steering parameters (`FlockingParams`), and agent rules.

## Novel Trait
The continuous `macroquad` simulation is wrapped with `arthropod`'s interactive layer. The swarm intelligence isn't static; by clicking discrete buttons, the user dynamically modulates the `FlockingParams` weights in real-time.

## Predicted Phenotype
An emergent, interactive playground. The chaotic swarming behavior can be instantly forced to scatter, align uniformly, or group into a dense ball through pure UI interaction, blending abstract graphical controls directly with physical biological models.

## Usage

```sh
cargo run -p arthropod-flock --release
```

*(Note: Use `cargo run -p arthropod-flock --release -- --headless` to safely bypass X11 UI panics in CI environments.)*
