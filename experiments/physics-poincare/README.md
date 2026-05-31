# physics-poincare

A TUI visualization crossing `physics-pbd` with `poincare-disk`. Rigid Euclidean bodies are simulated via Position Based Dynamics, then projected into the non-Euclidean Poincaré disk.

## Lineage

*   **crates/physics-pbd**: Provides the rigid-body physics constraints (PBD solver).
*   **crates/poincare-disk**: Provides the hyperbolic coordinate mapping.
*   **Novel Trait**: Physics works in standard Euclidean space, but visualization is compressed into the non-Euclidean disk, stretching and distorting objects exponentially as they move outward.

## Run

```bash
cargo run -p physics-poincare
```
