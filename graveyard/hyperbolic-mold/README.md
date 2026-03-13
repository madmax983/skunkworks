# Hyperbolic Mold

A hybrid experiment combining **Physarum Transit** (Slime Mold Simulation) with **Hyperbolic Browser** (Poincaré Disk Geometry).

## Concept

Agents (slime mold particles) live in the Poincaré Disk model of hyperbolic space. They move according to hyperbolic geometry rules (Möbius transformations), but deposit pheromones on a projected 2D map.

## Lineage

- **Parent A**: `experiments/hyperbolic-browser` (Environment: Poincaré Disk)
- **Parent B**: `experiments/physarum-transit` (Logic: Physarum Polycephalum)

## Novel Trait

**Hyperbolic Transport**: Agents follow geodesics in hyperbolic space. What looks like a straight line to the agent appears as a circular arc on the screen. The diffusion of pheromones occurs on the projection, creating a hybrid interaction between non-Euclidean movement and Euclidean diffusion.

## Controls

- None (Passive Simulation)

## Implementation Details

- **Agents**: 5000 particles with position `Complex<f64>` (Poincaré coords).
- **Movement**: Uses `poincare_disk::mobius_add` for geodesic translation.
- **Rendering**: `macroquad` texture updated via `rayon` parallelism.
