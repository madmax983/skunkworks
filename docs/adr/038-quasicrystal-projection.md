# 038. Crystal Defense: Hyper-dimensional Quasicrystal Projection

## Status
Accepted

## Context
The `crystal-defense` experiment aims to create a Tower Defense game set in an infinite, non-repeating dungeon. Standard game grids (Square, Hexagonal) are computationally efficient but topologically predictable. Players quickly learn optimal pathing strategies based on Manhattan or Euclidean distance.

We desired a game board that:
1.  **Exhibits Aperiodic Order:** Long-range order without translational symmetry (never repeats).
2.  **Possesses "Forbidden" Symmetry:** icosahedral (5-fold) symmetry, which is impossible in periodic crystals.
3.  **Challenges Spatial Reasoning:** Navigation requires following graph edges rather than cardinal directions.

Generating such structures procedurally using iterative rules (like Penrose tiling deflation) is complex in 3D. A more robust mathematical approach was needed.

## Decision
We decided to implement the **Cut-and-Project** method to generate 3D Icosahedral Quasicrystals from a 6D Hypercubic Lattice ($Z^6$).

### Algorithm
1.  **Higher Dimensional Space:** We define a 6D integer lattice where points are defined by 6 integers $(n_1, n_2, n_3, n_4, n_5, n_6)$.
2.  **Projection Matrices:** We define a projection matrix $P$ derived from the Golden Ratio ($\tau = \frac{1+\sqrt{5}}{2}$) that maps $R^6 \to R^3_{parallel}$ (Physical Space) and $R^6 \to R^3_{perp}$ (Perpendicular/Internal Space).
3.  **Selection Window:** We iterate over points in the 6D lattice. A point is "selected" (projected to 3D physical space) if and only if its projection into $R^3_{perp}$ falls within a specific geometric volume (the "Acceptance Window", typically a Triacontahedron).
4.  **Connectivity:** Edges are drawn between points that are nearest neighbors in the 6D lattice (distance 1).

## Consequences

### Positive
*   **Infinite Variety:** The lattice is deterministic but non-repeating. Every "room" (local configuration of nodes) is unique but follows strict geometric rules.
*   **Aesthetic Novelty:** The resulting structure resembles a 3D Penrose Tiling (Amman-Kramer-Neri tiling), providing a unique visual identity ("Sci-Fi/Alien" geometry).
*   **Graph-Based Gameplay:** The map is naturally a graph, not a grid. This simplifies pathfinding to Dijkstra/A* on edges but complicates spatial indexing.

### Negative
*   **Computational Cost:** The search space grows as $O(R^6)$ where $R$ is the grid radius. We must aggressively prune the search bounds to keep generation times interactive (< 1s for ~10k nodes).
*   **Navigation Complexity:** There is no simple arithmetic for "neighbor at $(x+1)$". All adjacency must be pre-calculated and stored in an adjacency list `Vec<Vec<usize>>`.
*   **Floating Point Precision:** The projection relies on irrational numbers ($\tau$). Identifying "identical" nodes or parallel edges requires epsilon-based comparisons, which can be fragile.
