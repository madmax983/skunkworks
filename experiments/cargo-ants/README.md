# Cargo Ants 🐜📦

A swarm intelligence simulation where ants resolve dependency graphs.

## Concept
- **Packages** are nodes in a DAG (Directed Acyclic Graph).
- **Dependencies** are edges.
- **Ants** are resolver agents. They traverse the graph to find valid paths to all leaves.
- **Pheromones** represent successful resolution paths.
- **Conflicts** (Red Nodes) kill ants, providing negative feedback (absence of pheromone return).

## Emergence
Watch as ants initially explore randomly. When one finds a path to a leaf node (valid dependency resolution), it returns to the root depositing pheromones. Subsequent ants follow these trails, optimizing the "resolution" process.

## Controls
- `R`: Reset the graph with a new random topology.
- `Hover`: See package details.

## Tech Stack
- **Rust**: Performance.
- **Macroquad**: 2D Visualization.
- **Rayon**: Parallel agent updates.
- **Petgraph**: Graph logic.
- **Glam**: Vector math.
