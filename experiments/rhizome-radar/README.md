# Rhizome Radar 🌿📡

**A Botanical Pathfinding Experiment**

> "Roots are the earth's neural network, seeking water with the precision of A* and the grace of a river." - Genesis

## The Concept

This experiment smashes together **Root Growth** and **Pathfinding Visualization**.
It uses the **Space Colonization Algorithm** (Runions et al., 2005) to simulate how roots explore soil.

- **The Rhizosphere (Open Set):** Nutrients emit attraction vectors.
- **The Root Tip (Agent):** Calculates the average direction of attraction.
- **The Result (Path):** A complex, branching dendritic structure that efficiently navigates the space.

## Controls

- **R**: Reset the simulation.
- **Space**: Toggle "Radar Mode" (visualize the sensing lines).
- **Left Click**: Feed the roots (add nutrient clusters).

## Algorithms

- **Space Colonization:** Roots grow by being attracted to nearby nutrients. Once a nutrient is reached (within a kill radius), it is consumed. This creates natural branching and competition for resources.
- **Radar Visualization:** We visualize the "attraction vectors" as faint lines, showing the plant's "knowledge" of the environment before it grows there.

## Tech Stack

- Rust 🦀
- Macroquad 🎨
- Rand 🎲
