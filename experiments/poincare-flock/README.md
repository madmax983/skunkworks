# poincare-flock

A TUI visualization crossing `poincare-disk` with `flocking`. Boids swarm within a non-Euclidean hyperbolic disk, compressing near the boundary.

## Lineage

*   **crates/flocking**: Provides the Boid swarm intelligence and rules (separation, alignment, cohesion).
*   **crates/poincare-disk**: Provides the hyperbolic coordinate mapping and Möbius addition.
*   **Novel Trait**: Boids use Euclidean space rules for their local perception, but traverse the environment using non-Euclidean mathematics. Thus, moving outward compresses them, simulating exponential distances.

## Run

```
cargo run -p poincare-flock
```
