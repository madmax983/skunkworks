# Origami Mycelium 🍄📜

**Lineage:** `crates/origami` × `experiments/myco-transit`

"The paper remembers the paths we walk."

## Concept
This experiment combines **deployable origami structures** (Miura-ori) with **slime mold pathfinding** (Physarum polycephalum).

The surface of the paper serves as a foraging ground for thousands of slime mold agents. As the agents traverse the paper and deposit pheromones, the chemical concentration physically contracts the paper's creases.

## Novel Trait
**Pheromone-Guided Folding:** The geometry of the origami mesh folds dynamically in response to biological intent. The paths the slime mold uses to commute between nodes cause the paper to fold and compress along those exact highways, creating a physical manifestation of the biological network.

## Controls
- **Mouse Drag**: Orbit Camera
- **Scroll**: Zoom in/out

## Tech Stack
- `origami`: Miura-ori mesh generation.
- Position Based Dynamics (PBD) for soft-body mesh simulation.
- `rayon` for parallel agent updates.
- `macroquad` for 3D rendering.