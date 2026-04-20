# 🌊 Poincaré Fluid

**Hyperbolic Magnetic Fluid**

> "The drop falls into the disk, and as it spreads to the edge, it reaches towards infinity."

## 🧬 Lineage
- **Parent A**: `crates/poincare-disk` (Hyperbolic Geometry, Non-Euclidean Distance)
- **Parent B**: `experiments/ferrous-fluid` (Magnetic Particle Fluid, Density Accumulation)
- **Concept**: A continuous 2D fluid simulation where the magnetic forces between particles are warped by the non-Euclidean geometry of a Poincaré disk.

## 🌿 Overview
This experiment drops the standard Euclidean distance calculations of our ferrous fluid simulation and replaces them with hyperbolic distance (`poincare_disk::hyperbolic_dist`).

As particles approach the boundaries of the space, the perceived magnetic distance between them approaches infinity, radically altering how the fluid clusters and repels itself.

## 🧪 Novel Trait
**Hyperbolic Space Compression:** Particles near the center flow freely as if in normal space, but as they are pushed outward by magnetic repulsion, the space "compresses". This causes incredibly dense clustering near the boundary since the magnetic forces drop off sharply (because the hyperbolic distance is so vast), yet visually they appear stacked on top of each other.
