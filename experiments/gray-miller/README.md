# gray-miller

A hybrid experiment combining `miller-lattice` and `gray-scott`.

## Concept: Reaction-Diffusion Codebase Crystals

**Lineage:**
- Inherits the 3D procedural generation of a file system crystal lattice from `miller-lattice`.
- Inherits the continuous chemical reaction-diffusion simulation from `gray-scott`.
- **Novel Trait:** The discrete crystalline file structure is squashed onto the 2D plane and serves as the initial "seed" points (spores of chemical V) in a continuous reaction-diffusion field. The codebase literally blooms and grows into Turing patterns.

## Running

```bash
cargo run -p gray-miller --release
```
