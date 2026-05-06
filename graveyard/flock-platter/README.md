# Flock Platter 🕊️🍽️

This experiment crosses `crates/flocking` with `crates/platter` to explore the concept of "Pheromone Swarming" or "Swarm Heatmaps".

## Lineage

*   **Parent A (flocking)**: Provides the Craig Reynolds' "Boids" algorithm for calculating swarm behavior (Separation, Alignment, Cohesion).
*   **Parent B (platter)**: Provides the continuous 2D scalar field representation and decay logic.
*   **Novel Trait**: Boids act as moving emitters that continuously saturate a 2D scalar field (platter). The field acts as a fading visual heatmap or "pheromone trail" for the swarm's activity over time. The boids themselves might also be influenced by the heatmap gradient.

## Emergent Phenotype
A swarm of entities that leaves behind a fading trail, showing the "memory" or "heat" of the swarm's previous paths, visualizing dense activity areas over time.
