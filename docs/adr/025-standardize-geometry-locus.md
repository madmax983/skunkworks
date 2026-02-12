# 025. Standardize 2D Geometry with Locus

## Status
Accepted

## Context
Many TUI-based experiments in this repository (`market-sim`, `chimera-sovereignty`, `automata-warfare`) involve 2D grids or continuous spaces that require vector mathematics and coordinate normalization (wrapping). Previously, each experiment implemented its own `Vec2` struct and its own logic for handling boundary conditions (wrapping vs. walls), leading to code duplication and subtle bugs in movement logic.

## Decision
We will create a lightweight, shared 2D geometry library called `crates/locus`.

### Key Components:
1.  **Vec2 Struct:** A robust 2D vector implementation supporting standard arithmetic (`Add`, `Sub`, `Mul`, `Div`), magnitude calculation, normalization, and reflection.
2.  **Topology Enum:** A declarative way to define the world's boundary behavior (e.g., `Plane`, `Torus`, `KleinBottle`, `Mobius`).
3.  **Normalization Logic:** A shared `normalize(y, x)` function that handles the complex logic of wrapping coordinates according to the selected topology.

## Consequences
### Positive
*   **Reduced Boilerplate:** Experiments no longer need to implement their own vector math.
*   **Consistent Physics:** Movement and collision logic will behave identically across different simulations.
*   **Advanced Topologies:** Experiments can easily switch between standard grids (Plane/Torus) and exotic geometries (Klein Bottle/Mobius) by changing a single enum variant.

### Negative
*   **Generality vs. Specificity:** The `Vec2` struct uses `f64`, which might be overkill for simple integer-grid simulations, potentially introducing minor overhead or casting requirements.
