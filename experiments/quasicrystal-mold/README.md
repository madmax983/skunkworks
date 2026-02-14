# Quasicrystal Mold

**Experiment:** `quasicrystal-mold`
**Status:** Fresh

## 🧬 Lineage

This experiment is a hybrid cross between:
- **Parent A:** `experiments/quasicrystal-dungeon` (The Lattice)
- **Parent B:** `experiments/bio-transit` (The Agents)

## 🔬 Concept

This simulation visualizes Physarum Polycephalum (slime mold) agents navigating an **Aperiodic Graph**.

Unlike standard Physarum simulations that run on a regular 2D pixel grid, these agents traverse the edges of a 3D Icosahedral Quasicrystal (projected from 6D).

### Key Mechanics
- **Lattice:** Generated via the "Cut and Project" method from a 6D hypercubic lattice to 3D.
- **Agents:** Commuter agents that oscillate between "Home" and "Work" nodes.
- **Pheromones:** Deposited on graph nodes and diffused to connected neighbors.
- **Pathfinding:** Probabilistic navigation based on pheromone gradients and directional bias.

## 🧪 Predicted Phenotype

The combination of bio-mimetic pathfinding and aperiodic geometry is expected to produce **non-repeating flow patterns**. The "highways" formed by the mold should follow the self-similar, fractal structure of the quasicrystal.

## 🎮 Controls

- **Left/Right Arrows:** Orbit the camera around the structure.
