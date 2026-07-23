# Arthropod Quipu 🐜🧶

**Concept:** Interactive Knotted Ledger.

This hybrid visualizer explores what happens when we cross the immediate-mode UI library of `arthropod` with the ancient knotted cord storage system of `quipu`.

## Lineage
- **Parent A (crates/arthropod):** Provides the interactive immediate mode graphical UI buttons and color logic.
- **Parent B (crates/quipu):** Provides the discrete structural memory architecture that encodes data dynamically as permanent tied knots along a cord.

## Novel Trait
The ancient, abstract data storage cord is wrapped with `arthropod`'s interactive layer. By clicking discrete UI buttons, the user dynamically injects integers that are physically tied as structural knots onto a central quipu cord, providing a direct visual mapping between abstract interface actions and ancient physical data structures.

## Predicted Phenotype
An emergent, interactive archaeological playground. Abstract UI buttons instantly tie and accumulate physical knots along a main cord.

## Usage

```sh
cargo run -p arthropod-quipu --release
```

*(Note: Use `cargo run -p arthropod-quipu --release -- --headless` to safely bypass X11 UI panics in CI environments.)*
