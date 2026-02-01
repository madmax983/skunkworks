# Git Galaxy 🌌

Git Galaxy is a visualization experiment that renders your git repository history as a force-directed graph in the terminal.

## How it works

- **Nodes**: Each commit is a "star" in the galaxy.
- **Edges**: Parent-child relationships connect the stars.
- **Mass**: The "churn" (insertions + deletions) of a commit determines its mass and gravitational pull.
- **Color**: Generated deterministically from the author's name.

## Physics

The simulation uses a custom physics engine with:
1. **Coulomb Repulsion**: Pushes all nodes apart to prevent overlapping.
2. **Hooke's Law (Springs)**: Pulls connected commits together.
3. **Damping**: Simulates friction to stabilize the graph over time.

## Usage

Run from the repo root:

```bash
cargo run -p git_galaxy -- [path_to_repo]
```

If no path is provided, it defaults to the current directory.

Controls:
- `q`: Quit
