# Arthropod Poincare 🐜🥏

**Concept**: Interactive Non-Euclidean Sandbox.

This hybrid visualizer explores what happens when we cross the immediate-mode UI library of `arthropod` with the continuous hyperbolic non-Euclidean geometry of `poincare-disk`.

## Lineage
- **Parent A (crates/arthropod):** Provides the interactive immediate mode graphical UI buttons and color logic.
- **Parent B (crates/poincare-disk):** Provides the mathematical primitives for hyperbolic space, Möbius transformations, and coordinate logic for a continuous disk.

## Novel Trait
The continuous non-Euclidean simulation is wrapped with `arthropod`'s interactive layer. Users dynamically modulate the geometric bounds of the universe in real-time using UI buttons to continuously apply Möbius transformations (translations and rotations) to a `{4, 5}` hyperbolic tiling representation.

## Predicted Phenotype
An emergent, interactive sandbox where users explore the infinite bounds of the Poincaré disk. The rigid mathematical structures are manipulated manually by interactive UI inputs.

## Usage

```sh
cargo run -p arthropod-poincare --release
```

*(Note: Use `cargo run -p arthropod-poincare --release -- --headless` to safely bypass X11 UI panics in CI environments.)*
