# locus-flock 🧬

A hybrid experiment crossing `crates/locus` with `crates/flocking`.

## Lineage
- **crates/locus**: Provides the `Topology` enum and logic to wrap coordinates in non-Euclidean space (e.g. Torus, Klein Bottle).
- **crates/flocking**: Provides the Boids simulation logic (`compute_force` and `FlockingParams`).

## Phenotype
By projecting the boids agent locations onto the topological plane, we can wrap distances. Swarms navigating a Torus will see agents on the opposite edge as right next to them, creating "ghost" separation/alignment forces from beyond the boundary.
