# `locus-fluid`

A hybrid experiment crossing the Boid flocking mechanics of `locus` with the magnetic fluid simulation of `ferrous-fluid`.

## Lineage

- **Parent A:** `crates/locus` (Flocking physics via `compute_force`)
- **Parent B:** `experiments/ferrous-fluid` (Continuous magnetic pressure via `Platter` and `Magnet` structs)

## Phenotype

**Magnetic Boid Flocking.**
Particles use classic Boid flocking rules (Separation, Alignment, Cohesion) to navigate, but they also act as magnetic poles in a continuous fluid simulation. Their movement is guided by both the discrete flocking forces and the continuous magnetic fields. This creates a macroscopic-microscopic feedback loop where the boids shape the magnetic field, and the magnetic field shapes the boids' flocking, resulting in amoeba-like swarms that stretch along field lines.