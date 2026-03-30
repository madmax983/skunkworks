# Neuro-Flock 🧠🐦

Spiking Neural Boids.

A hybrid experiment combining the macroscopic flocking behavior of `luminous-flock` with the microscopic biological neuron simulation of `neuro-sim`.

## Concept

In standard flocking simulations, boids use pure physics vectors (alignment, cohesion, separation) to determine their steering forces. **Neuro-Flock** replaces these hardcoded physics rules with an embedded biological brain.

Each boid contains a `neuro_sim::Network` of Izhikevich spiking neurons.
*   **Sensory Neurons**: The distances and velocities of neighbors are broken down into positive and negative X/Y components. These values are injected as electrical current into the sensory neurons.
*   **Motor Neurons**: The output spikes of the motor neurons directly determine the steering impulses applied to the boid.

This creates a system where the macroscopic flocking behavior is entirely driven by the discrete, non-linear spiking dynamics of the underlying neural networks.

## Lineage

*   **Parent A:** `experiments/luminous-flock` (Boid Flocking & Rendering)
*   **Parent B:** `crates/neuro-sim` (Izhikevich Neuron Model)
*   **Novel Trait:** Neural Swarming. The continuous swarming intent is processed through microscopic biological spiking networks, bridging individual neural computation and emergent collective physical behavior.

## Controls

*   `Q` / `Esc`: Quit
*   `R`: Reset World
