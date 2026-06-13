# flock-poincare

A hybrid experiment combining swarm intelligence (`flocking`) with non-Euclidean hyperbolic geometry (`poincare-disk`).

## Lineage

* **From `flocking`**: The continuous particle swarm mechanics, including separation, alignment, and cohesion forces.
* **From `poincare-disk`**: The hyperbolic coordinate system, spatial mapping, and boundary constraint logic.

## Phenotype

Boids navigate within the continuous hyperbolic space of the Poincaré disk. As they approach the infinite boundary, distances warp, and their physical movement is mathematically bounded, generating an endlessly dense swarming pattern at the edges.

## Running

```bash
cargo run -p flock-poincare
```
