# Hive Mind Dependencies 🐜📦

An experiment visualizing **Dependency Resolution** as an **Ant Colony Optimization** (ACO) process.

## Concept

Software dependencies form a Directed Acyclic Graph (DAG). Resolving them (compilation) requires traversing this graph.
In this simulation:
- **Terrain**: The dependency graph.
- **Ants**: Builder threads seeking "leaves" (dependencies with no unbuilt dependencies).
- **Pheromones**: Mark the "hot paths" or critical chain of the build.
- **Artifacts**: Ants return with "compiled artifacts" (food).

## Technical Details

- **Stack**: Rust, `ratatui` (TUI), `petgraph` (Graph), `rayon` (Parallelism - future).
- **Visualization**:
  - **Nodes**: Crates.
  - **Edges**: Dependencies. Color indicates pheromone intensity (traffic).
  - **Ants**: Dots moving along edges. Green = Carrying artifact, Magenta = Foraging.

## Usage

```bash
cargo run -p hive-mind-dependencies
```

Controls:
- `q`: Quit.

## Future Work

- [ ] Dynamic graph mutation (breakages, new dependencies).
- [ ] Parallelize ant updates with `rayon`.
- [ ] 3D visualization using `bevy`.
