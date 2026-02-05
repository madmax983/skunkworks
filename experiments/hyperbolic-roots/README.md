# Hyperbolic Roots 🧬

A hybrid experiment combining the **Space Colonization Algorithm** (from `rhizome-radar`) with **Hyperbolic Geometry** (from `hyperbolic-space`).

## Concept

Roots grow in the Poincaré Disk model of hyperbolic space. They seek "nutrients" (points in the disk), but distance and growth direction are calculated using hyperbolic metrics.

Because space expands exponentially towards the edge of the disk:
- Roots branch more aggressively to cover the "larger" space near the edge.
- Geodesics (growth paths) follow circular arcs orthogonal to the boundary.

## Lineage

- **Parent A**: `experiments/rhizome-radar` (Algorithm: Space Colonization)
- **Parent B**: `experiments/hyperbolic-space` (Concept: Non-Euclidean Geometry)
- **Donor**: `experiments/hyperbolic-ants` (Math: Mobius transformations)

## Controls

- `R`: Reset simulation
- `Space`: Toggle nutrient visibility
- `Click`: Add nutrients (at mouse position)
