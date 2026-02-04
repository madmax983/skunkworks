# Hyperbolic Ants 🐜♾️

A hybridization of `hyperbolic-dungeon` and `cargo-ants`.

> "The colony has expanded beyond Euclidean bounds. The pheromones follow geodesics." - The Splice Surgeon

## Concept

Ants foraging for food on an infinite {4,5} hyperbolic tiling (Poincaré Disk model).
The space is non-Euclidean, meaning the area grows exponentially as the ants explore outwards.

## Lineage

*   **Parent A:** `hyperbolic-dungeon` (Infinite hyperbolic Tiling generation, Mobius geometry)
*   **Parent B:** `cargo-ants` (Ant agent behavior, pheromone trails)

## Novel Traits

*   **Hyperbolic Pheromones:** Scent trails decay and diffuse across a non-Euclidean grid.
*   **Infinite Foraging:** The ants can explore forever without hitting a boundary, but getting back to the nest requires navigating the hyperbolic tree structure.

## Controls

*   **WASD / Arrows**: Move Camera
*   **Q / Esc**: Quit

## Legend

*   **Yellow Dot**: Ant foraging
*   **Magenta Dot**: Ant carrying food
*   **Red Circle**: Food source
*   **Green Tint**: Pheromone trail
*   **Cyan Dot**: Camera center
