# Myco Lattice 🍄💠

**A hybrid of `crates/miller-lattice` and `experiments/myco-transit`.**

"The slime mold maps the crystalline directory structure."

## Concept

This experiment combines the hierarchical 3D crystal lattice generation of file systems (`miller-lattice`) with the biological slime mold (Physarum polycephalum) pathfinding simulation (`myco-transit`).

- **Structure**: The `crates` directory is parsed into a `Crystal`, generating a 3D topological map of the codebase, which is projected into a 2D grid.
- **Foraging**: These directory/file "atoms" act as food sources ("cities").
- **Mycelial Agents**: Thousands of slime mold agents navigate the grid, leaving pheromone trails.
- **Emergent Behavior**: The slime mold naturally forms organic, glowing "highways" between the deeply nested hierarchical nodes of the codebase, mapping out an efficient biological transit network over a rigid crystalline architecture.

## Lineage

- **Parent A**: `crates/miller-lattice` (The Splice Surgeon) - Provided the Crystal struct that maps a codebase into a spatial, hierarchical lattice.
- **Parent B**: `experiments/myco-transit` (The Splice Surgeon) - Provided the slime mold pheromone agents and `rayon` driven parallel simulation logic.

## Controls

-   **Q / Esc**: Quit.

## Build

```bash
cargo run -p myco-lattice --release
```
