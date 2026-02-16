# Algo-Botany 🌿🧠

**"The Roots of Knowledge"**

> "A* is just a root system seeking water with optimal efficiency." - Genesis

## The Concept

This experiment explores the visual similarities between graph traversal algorithms (like A*) and organic root growth.
We visualize the **Open Set** as budding root tips and the **Came From** map as the established root network.

The soil grid has varying density (though currently implemented as binary Wall/Soil for simplicity), representing the cost function.

## Algorithms as Plants

- **A* (A-Star)**: The taproot. It drives directly towards the goal (Water) using a heuristic, branching only when necessary to navigate obstacles.
- **Dijkstra**: The moss. It spreads uniformly, exploring all paths equally until it finds water. (Currently implemented as A* with h=0).
- **Greedy Best-First**: The vine. It shoots recklessly towards the goal, often getting stuck in dead ends (local minima).

## Controls

- **Left Click**: Place Obstacle (Rock).
- **Right Click**: Place Water (Goal).
- **S + Left Click**: Place Seed (Start) and reset.
- **Space**: Pause/Resume growth.
- **R**: Reset the plant (clear roots, keep walls).

## Implementation Details

- **Language**: Rust 🦀
- **Engine**: Macroquad 🎨
- **Logic**:
  - `Grid`: Stores cell types and costs.
  - `Plant`: Manages the priority queue (`BinaryHeap`) and path reconstruction.
  - `Heuristic`: Manhattan distance for A*.

## Future Ideas

- Different soil densities (Hard Soil vs Soft Soil).
- Multiple root systems competing for water.
- "Nutrient" packets that encourage branching.
