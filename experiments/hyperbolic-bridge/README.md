# 🧬 Hyperbolic Bridge

**Lineage:** `biomimetic-bridge` × `hyperbolic-mold`

## Concept
Ants traversing the Poincaré Disk encounter a "Moat of Infinity" (a ring gap). To cross it, they must form living bridges. This experiment combines the swarm intelligence of `biomimetic-bridge` with the non-Euclidean transport of `hyperbolic-mold`.

## Emergent Traits
- **Infinite Bridges:** Bridges must span hyperbolic distances. The curvature of space means straight bridges look like arcs.
- **Hyperbolic Swarm Intelligence:** Agents use pheromone trails that diffuse according to Euclidean rules on the projected disk, creating a hybrid interaction model.

## Implementation
- **Math:** `poincare-disk` crate for Möbius transformations and hyperbolic distance.
- **Vis:** `macroquad` for rendering the disk and agents.
- **Logic:** Agents freeze into `Bridging` state when encountering a void with sufficient local support (density).

## Controls
- Run with `cargo run -p hyperbolic-bridge`
