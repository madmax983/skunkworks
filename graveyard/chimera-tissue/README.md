# Chimera Tissue 🧬

**Parents**: `chimera-genesis` (Concept) + `physics-pbd` (Physics)

A "Programmable Matter" simulation where a 2D soft-body mesh is driven by a colony of ChimeraVM agents.

## Concept

Each particle in the physics simulation represents a biological cell containing a nucleus (ChimeraVM). The cells are connected by "Actuator" constraints (muscle fibers) that can contract or expand.

### The Feedback Loop

1.  **Sense**: The cell measures the strain (stress) on its connected muscle fibers.
2.  **Think**: The ChimeraVM executes its genetic code (DNA), processing the strain input.
3.  **Act**: The VM outputs a target contraction factor, which physically changes the resting length of the muscle fibers.

This closed-loop system allows for the emergence of:
- Peristaltic movement
- Rhythmic pulsation (heartbeat)
- Stimulus-response behaviors (cringing when stretched)

## Implementation

- **Physics**: Uses `physics-pbd` (Position Based Dynamics) for stable soft-body simulation.
- **Biology**: Uses `chimera-lang` for the genetic programming of cell behavior.
- **Visualization**: Uses `macroquad` for rendering.

## Lineage

Inherits the "Cellular Automata with VMs" concept from `chimera-genesis` but replaces the discrete grid with a continuous physical mesh, inspired by `neuro-fold` and `origami-swarm`.
