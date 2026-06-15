# Miller-Locus 📍🗂️

A topological filesystem lattice.

## Concept
This experiment projects the discrete 3D crystalline lattice of a filesystem structure (`miller-lattice`) down onto a continuous 2D non-Euclidean boundary (`locus`).

## Lineage 🧬
- **Parent A (`crates/miller-lattice`)**: Provides the procedural generation of a hierarchical crystalline lattice from filesystem data using discrete Miller index coordinates.
- **Parent B (`crates/locus`)**: Provides the non-Euclidean topological boundary structures (`Torus`, `KleinBottle`) that dictate how space warps when bounds are exceeded.

## Emergent Phenotype
Instead of branching infinitely into a void, large filesystem hierarchies are constrained by the topological map. When deep directory structures push past the environment's boundary, they seamlessly wrap around, interweaving discrete file branches back upon the origin like a folded Torus.

## Execution
Run headless to verify mathematical projection:
```bash
cargo run -p miller-locus -- --headless
```
