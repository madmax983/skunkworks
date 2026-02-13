# Hive Architecture 🐝🏗️

> "What if we let termites design our data centers?"

A **Moonshot** experiment combining **Termite Mound Ventilation** (Stigmergy) and **Distributed Consensus** (Political Architecture).

## Concept
We simulate a swarm of 10,000+ "Architect Termites" inhabiting a server rack. The rack generates heat. The termites want to cool it down.
*   **Physics:** Heat diffuses through the grid. Walls block heat (insulation) but also block airflow (cooling).
*   **Biology:** Termites have a "Blueprint" (genetic parameters) determining when to build or destroy walls based on local temperature.
*   **Politics:** Termites share their Blueprints. If a termite is in a "cool" (successful) area, it broadcasts its Blueprint. Termites in "hot" (failing) areas adopt the Blueprint of successful neighbors.

## Emergence
Watch as the swarm:
1.  **Explores:** Randomly building walls.
2.  **Learns:** Successful cooling strategies (chimneys, heat sinks) spread through the population.
3.  **Converges:** The entire structure morphs into an optimized organic cooling machine.

## Controls
*   **Run:** `cargo run --release`
*   **Visuals:**
    *   **Blue-Red Gradient:** Heat Map (Blue = Cool, Red = Hot).
    *   **White Pixels:** Walls.
    *   **Colored Dots:** Termites (Color represents their unique Blueprint/Tribe).

## Tech Stack
*   `macroquad`: Visualization.
*   `rayon`: Parallel processing of the heat diffusion grid.
*   `fastrand`: Deterministic randomness.
