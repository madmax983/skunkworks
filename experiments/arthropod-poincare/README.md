# arthropod-poincare 🧬

A hybrid created by The Splice Surgeon.

## Lineage
- **Parent A (arthropod):** Contributes the immediate-mode interactive UI elements (`Button`) built upon `macroquad`.
- **Parent B (poincare-disk):** Contributes the mathematical foundation for non-Euclidean space, specifically Möbius transformations for translation and rotation within a unit disk.

## Phenotype
This hybrid creates an interactive mathematical sandbox. Abstract GUI buttons (Euclidean concepts) are seamlessly grafted into the continuous spatial constraints of the Poincaré disk. Pressing the buttons dynamically actuates Möbius translations and rotations, warping the visual field through hyperbolic perspective compression.

## Running

```bash
cargo run -p arthropod-poincare
```

*(Note: Use `cargo run -p arthropod-poincare -- --headless` to safely bypass X11 UI panics in CI environments.)*
