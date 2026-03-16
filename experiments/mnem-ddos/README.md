# Mnem-DDoS

**Experiment Type:** Swarm Intelligence / Codebase Entropy Visualization
**Status:** [FRESH]
**Stack:** Rust, Macroquad, Rayon

## 🧬 Splice Lineage

This experiment is a hybrid cross between `locust-ddos` and `mnem-rot`.

### Parent Alleles
- **From `locust-ddos`:** The macroscopic swarm intelligence mechanics, using thousands of agents updating in parallel to simulate a cyberattack vector.
- **From `mnem-rot`:** The codebase neural-network representation (files and `use` dependencies) acting as nodes, health states, and a stochastic text corruption algorithm (digital entropy).

### Novel Phenotype: Entropy-Driven Cyberwarfare
Instead of an abstract target, the botnet swarm targets lines of code and specific files (nodes) in the codebase. As agents swarm a node, they "rot" the data. The codebase graph's entropy is directly driven by parasitic swarm agents, creating a tug-of-war between the swarm's chaotic consumption of nodes and the user's active maintenance. Nodes with low health will glitch when you hover over them to view their text.

## Controls

- **Hover:** Heal a node and view its (potentially corrupted) text content. Healing a node repels the rotting effects of the swarm.
- **Right Click + Drag:** Pan the camera around the codebase graph space.
- **Scroll:** Zoom in and out.

## Technical Details

- **Agents:** Swarm simulated via `rayon` parallel updates.
- **Physics:** Graph layout is simulated with continuous spring and repulsive forces.
- **Text Glitch:** Stochastic bit-rot, case flipping, and text replacement simulation mapped to a node's health intensity.