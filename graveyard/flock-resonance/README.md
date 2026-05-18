# Flock Resonance 🕊️🔊

A hybrid experiment crossing the `flocking` (Boids) algorithm with `resonance-audio` (FDTD acoustic wave simulation).

## Concept: Acoustic Swarm Morphogenesis

This experiment translates the behavioral dynamics of a swarming organism into continuous acoustic pressure waves. The continuous 2D boids simulation acts as a dynamic, moving acoustic exciter within a continuous 2D acoustic wave tank simulation.

As the "boids" swarm and move through the tank, they "pluck" the acoustic space. The density, speed, and cohesion of the flock directly dictate the intensity and frequency of the sound waves produced, creating an emergent sonification of their movement. The boundaries of the tank also reflect and interfere with these waves, producing complex cymatic patterns from simple swarming rules.

## Lineage

*   **From `crates/flocking`**: The core Boids simulation logic governing separation, alignment, and cohesion of the agents.
*   **From `crates/resonance-audio`**: The Finite Difference Time Domain (FDTD) solver for the 2D acoustic wave equation.
*   **Novel Trait**: Binding the velocity and position of the discrete flocking agents to the continuous pressure injection of the acoustic grid, allowing the swarm to dynamically generate and interact with physical sound waves.
