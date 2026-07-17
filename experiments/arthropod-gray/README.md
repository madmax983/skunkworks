# Arthropod Gray 🐜🧪

**Concept:** Interactive Chemical Morphogenesis.

This hybrid visualizer explores what happens when we cross the immediate-mode UI library of `arthropod` with the continuous chemical reaction-diffusion simulation of `gray-scott`.

## Lineage
- **Parent A (crates/arthropod):** Provides the interactive immediate mode graphical UI buttons and color logic.
- **Parent B (crates/gray-scott):** Provides the continuous thermodynamic reaction-diffusion simulation.

## Novel Trait
The continuous morphogenetic simulation is wrapped with `arthropod`'s interactive layer. The thermodynamic rules aren't static; by clicking discrete buttons, the user dynamically modulates the `feed` and `kill` chemical rates in real-time.

## Predicted Phenotype
An emergent, interactive playground. The continuous biological patterns can be instantly forced to change from spots to stripes, or dissipate entirely, through pure UI interaction, blending abstract graphical controls directly with morphogenetic biological models.

## Usage

```sh
cargo run -p arthropod-gray --release
```

*(Note: Use `cargo run -p arthropod-gray --release -- --headless` to safely bypass X11 UI panics in CI environments.)*
