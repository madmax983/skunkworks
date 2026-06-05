# Quipu-Locus: Topological Data Knots

A hybrid combining `crates/quipu` and `crates/locus`.

## Concept
The discrete knotted data structures of the ancient Inca Quipu are mapped into the continuous topological boundaries of `locus` (like a Torus or Klein Bottle). This allows integer states to wrap seamlessly across non-Euclidean boundary layers, producing spatial distribution of encoded memory.

## Lineage
- `quipu`: Provides the structural Cord and Knot data types, storing integers physically.
- `locus`: Provides the non-Euclidean boundary rules (`Topology`).

## Phenotype
Discrete data clusters wrap across the Euclidean bounds of the rendering screen, emerging on the opposite sides according to the geometry of the chosen topological space.

## Running
```
cargo run -p quipu-locus
```
