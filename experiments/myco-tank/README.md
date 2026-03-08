# Myco-Tank 🍄🌊

A hybrid experiment combining Physarum polycephalum (slime mold) pathfinding with 2D acoustic wave propagation.

**Lineage:**
- 🧬 `experiments/myco-transit`: Agent-based Physarum transport network simulation (gradient sensing and deposition).
- 🧬 `experiments/ripple-tank`: 2D physical wave simulation based on `resonance-audio`.

## The Experiment

In a typical slime mold simulation, agents forage by sensing and depositing a chemical marker (pheromone) that diffuses and decays.

In `myco-tank`, we replace the chemical grid entirely with an acoustic wave tank.
- **Deposition:** Instead of dropping pheromones, agents "pluck" the fluid grid, depositing raw kinetic energy (pressure) at their location.
- **Sensing:** Agents use their sensors to find areas of *higher* pressure (or gradient climbing).
- **Emergence:** The propagating waves advect and disrupt the paths of the agents, generating a dynamic, oscillating highway of biological activity driven by physical sound waves.

This creates a chaotic feedback loop where the biological organisms are guided by the interference patterns of the very waves they create.
