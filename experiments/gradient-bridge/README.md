# Gradient Bridge 🧬

> "Optimization is a journey, not just a descent."

A hybrid experiment combining the mathematical landscapes of `gradient-garden` with the swarm intelligence of `biomimetic-bridge`.

## Concept

Ants traverse a mathematical optimization landscape (Rosenbrock, Rastrigin, etc.) performing gradient descent to find the global minimum (deepest valley). However, local minima (shallow valleys) trap them.

When ants get stuck in a local minimum that is not the global goal, they transform into "Bridge" states, effectively filling the valley with their bodies. This raises the local terrain height, allowing subsequent ants to walk over the filled depression and continue their search for the true global minimum.

This is a visualization of **Stigmergic Optimization** or **Basin Hopping via Swarm Construction**.

## Lineage

*   **Parent A**: `experiments/gradient-garden` (Objective Functions, Heatmap Visualization)
*   **Parent B**: `experiments/biomimetic-bridge` (Ant Agents, Bridging Behavior)
*   **Novel Trait**: Physicalizing optimization heuristics. "Momentum" becomes "Falling". "Tunneling" becomes "Bridging".

## Controls

*   **Space**: Switch Objective Function (Gaussian Hills -> Rastrigin -> Rosenbrock -> Ackley -> EggHolder)
*   **R**: Reset Simulation

## Visuals

*   **Heatmap**: Blue/Green/Yellow gradient representing the function value.
*   **Red Dots**: Foraging Ants (Gradient Descent).
*   **Blue Dots**: Bridging Ants (Static, Filling Local Minima).
