# Luminous Tank 🌊✨

**A Hybrid Experiment by The Splice Surgeon**

> "Acoustic Swarming"

## 🔬 Experiment Analysis

This experiment combines flocking behavior with acoustic wave dynamics.

- **Parent A**: `crates/locus` (Flocking Behavior and Topology)
- **Parent B**: `experiments/ripple-tank` (2D physical wave tank synthesis)

### Concept
Standard boids move according to visual rules (Reynolds). In **Luminous Tank**, boids exist on a 2D acoustic grid. As they move, they deposit energy into the grid, creating physical pressure waves. These waves, in turn, advect and push the other boids, creating a bidirectional feedback loop between the macroscopic swarm and the physical acoustic medium.

### Lineage & Genetics
- **Allele A (Motion)**: Flocking physics from `crates/locus` (Separation, Alignment, Cohesion).
- **Allele B (Medium)**: 2D wave equation and pressure field from `experiments/ripple-tank`.
- **Emergent Trait**: Acoustic Swarming. The visual form of the swarm becomes clustered by its own generated sound waves.

## 🕹️ Controls

- **Left Click**: Manually pluck the water surface
- **Space**: Clear all waves
- **C**: Clear all walls
- **R**: Reset simulation
- **Q**: Quit

## 📊 Technical Details

- **Agents**: Simulated using `rayon` for parallel processing.
- **Wave Tank**: Solves the 2D wave equation in real-time, feeding audio to the output stream.
- **Coupling**: Particles inject pressure upon events; pressure gradients compute forces back to the particles.
