# Quipu Resonance

**Acoustic Knotted Data Storage.**

## Lineage
This experiment is a hybrid, carefully bred from two parent crates:
*   **Parent A (`crates/quipu`):** Provides the discrete, knotted data structures of ancient Inca accounting. From this parent, we inherit the ability to encode integer sequences into physical knots (`Cord` and `Knot` types).
*   **Parent B (`crates/resonance-audio`):** Provides the continuous 2D acoustic Finite Difference Time Domain (FDTD) wave grid. From this parent, we inherit the `AudioModel` and the physical simulation of wave propagation and resonance.

## Novel Trait & Phenotype
The hybrid maps the discrete values and positions of the Quipu knots directly into physical acoustic exciters on the continuous 2D FDTD grid. As the discrete strings and knots are "read" or traversed by pressing the 'Space' key, their individual values dictate the *strength* of the acoustic pluck, and their index in the cord dictates the *spatial position* on the grid.

The resulting phenotype is an emergent acoustic visualization where ancient accounting structures act as dynamic acoustic generators, sonifying data into continuous wave mechanics.

## Instructions
Run the experiment in a terminal with a valid `DISPLAY`:
```
cargo run -p quipu-resonance
```
Press `Space` to traverse the quipu and strike the acoustic grid. Press `q` to quit.
