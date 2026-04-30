# gray-poincare

A hybridization of `crates/gray-scott` and `crates/poincare-disk`.

## Lineage
- **gray-scott (Parent A):** Provides the continuous 2D chemical reaction-diffusion simulation.
- **poincare-disk (Parent B):** Provides hyperbolic non-Euclidean mathematics and spatial projection.

## Phenotype
**Hyperbolic Reaction-Diffusion**

In this experiment, the standard Euclidean grid of the Gray-Scott Turing patterns is visually projected into the Poincaré disk. The chemical diffusion occurs on a uniform grid, but the visual rendering treats the coordinate space as hyperbolic.

As the reaction spreads from the center outwards, the expanding perimeter is visually squashed and compressed according to the non-Euclidean distance metric of the disk ($d = \text{atanh}(r)$). The visual density of the chemical patterns increases exponentially as they approach the boundary of the disk (infinity).
