# Chimera Cam 🧬⚙️

**Genesis: The Splice Surgeon**

A hybrid experiment combining **Genetic Programming** (`chimera-lang`) with **Mechanical Physics** (`cam-automaton`).

## Concept
**"The Gene Shapes the Machine."**

Agents in this simulation are `ChimeraVM` instances. Their genetic code (DNA) calculates the geometry of a mechanical cam.
This cam drives a physics-simulated puppet (ragdoll). The fitness of the agent is determined by how well the cam drives the puppet (e.g., maximum lift).

## Mechanism
1.  **Genetic Design Phase**: Each agent executes its DNA. The values pushed to the stack are interpreted as Polar Coordinates `(Radius, Angle)`.
2.  **Fabrication Phase**: These points form a Convex Hull, creating a physical Cam shape.
3.  **Simulation Phase**: The cam is attached to a rotating shaft. Physics (`rapier2d`) simulates the interaction between the cam, followers, and the puppet.
4.  **Evolution**: Agents that lift the puppet higher reproduce and mutate.

## Controls
- Watch the **Best Agent** of the current generation evolve in real-time.
- **Generation** and **Fitness** stats are displayed on screen.

## Lineage
- **Parent A**: `experiments/chimera-lang` (DNA, VM, Evolution)
- **Parent B**: `experiments/cam-automaton` (Physics, Visualization, Linkages)
- **Novel Trait**: Evo-Mechanical Design. Code becomes physical structure.
