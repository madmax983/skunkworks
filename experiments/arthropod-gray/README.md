# Arthropod Gray-Scott 🦠

**Concept:** Interactive Morphogenetic Engineering.

This hybrid visualizer explores what happens when we cross the immediate-mode UI library of `arthropod` with the thermodynamic Turing pattern simulation of `gray-scott`. Instead of a passive chemical observation, the user has direct control over the reaction rates and manually paints chemical injections.

## Lineage
- **Parent A (crates/arthropod):** Provides the interactive immediate mode graphical UI buttons and mouse tracking logic.
- **Parent B (crates/gray-scott):** Provides the robust, grid-based reaction-diffusion engine (Gray-Scott model) simulating chemical U and V interactions.

## Novel Trait
The continuous `macroquad` simulation is wrapped with `arthropod`'s interactive layer. By clicking discrete buttons, the user dynamically modulates the `feed_rate` and `kill_rate` in real-time, observing how the chemical landscape fundamentally morphs its ruleset on the fly. Furthermore, the user acts as a biological injector, dropping chemical V directly into the environment.

## Predicted Phenotype
An emergent, interactive laboratory. The user can sculpt Turing patterns in real time, pushing the system to the brink of chemical chaos, cell division, or total extinction, all driven through pure UI interaction.

## Usage

```sh
cargo run -p arthropod-gray --release
```

*(Note: Use `cargo run -p arthropod-gray --release -- --headless` to safely bypass X11 UI panics in CI environments.)*
