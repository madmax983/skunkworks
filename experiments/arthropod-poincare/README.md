# Arthropod Poincare 🐜💠

**Concept:** Interactive Hyperbolic Navigation.

This hybrid visualizer explores what happens when we cross the immediate-mode UI library of `arthropod` with the continuous non-Euclidean geometry of `poincare-disk`.

## Lineage
- **Parent A (crates/arthropod):** Provides the interactive immediate mode graphical UI buttons.
- **Parent B (crates/poincare-disk):** Provides the continuous non-Euclidean mathematics and Möbius transformations.

## Novel Trait
The abstract continuous geometry is now interactively explorable. Discrete UI button clicks apply continuous Möbius transformations (translations and rotations) to the view center, allowing the user to navigate the hyperbolic plane in real time.

## Predicted Phenotype
An interactive non-Euclidean kaleidoscope. As the user clicks the UI buttons to move around, the underlying Euclidean representation squashes and stretches according to hyperbolic metrics, bridging discrete GUI actions to non-Euclidean continuous spaces.
