# gray-poincare

**Lineage:** `crates/gray-scott` × `crates/poincare-disk`

## Concept
Hyperbolic Reaction-Diffusion. This hybrid maps continuous chemical Turing patterns (V concentration from the `gray-scott` simulation) into the continuous non-Euclidean geometry of the Poincaré disk.

## Novel Trait
Instead of a standard flat 2D grid, the visual representation applies a mapping that transforms the screen space into the non-Euclidean geometry of the `poincare-disk` via Mobius-like positional mapping. As the chemicals diffuse, they appear to stretch infinitely toward the boundary of the disk.

## Emergence
The visual representation effectively bounds an infinite plane into a finite disk on the screen, creating an intricate biological simulation that shrinks toward the edges and expands at the center.
