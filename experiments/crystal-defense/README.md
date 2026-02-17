# 💎 Crystal Defense

**Lineage**: `quasicrystal-dungeon` × `thermo-defense`

A Tower Defense game played on a 6D-to-3D projected Icosahedral Quasicrystal lattice.

## Concept
The "Server" resides at the center of the crystal. "Locusts" spawn at the periphery (leaf nodes) and attempt to reach the center to destroy it. "Termites" (Defenders) spawn at the center and move out to build defenses and intercept attackers.

## Mechanics
- **Graph Topology**: The world is not a grid, but an aperiodic graph.
- **Heat Diffusion**: Activity generates heat which diffuses across the graph edges. Termites must cool down the lattice.
- **Pheromones**:
  - **Attack Pheromone**: Dropped by Locusts, guiding others to the center.
  - **Defense Pheromone**: Dropped by Termites, marking patrolled areas.

## Controls
- **WASD / Shift+Space**: Move Camera.
- **Cursor**: Use `Tab` to cycle neighbors, `Enter` to move cursor.
- **Esc**: Exit.

## Implementation Details
- **Simulation**: Parallelized agent updates and heat diffusion using `rayon`.
- **Rendering**: `wgpu` instanced rendering of atoms and edges. Nodes glow based on heat and pheromone levels.
