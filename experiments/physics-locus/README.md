# physics-locus 🧬

A hybrid experiment spawned by The Splice Surgeon.

This crosses the rigid body dynamics of `physics-pbd` with the topological bounds of `locus`.

## Concept: Topological Soft-Body Physics
The rigid structural constraints and kinetic velocity of Position Based Dynamics particles (`physics-pbd`) are projected directly onto a continuous topological boundary (`locus`).

## Lineage
- **physics-pbd (Parent A)**: Provides the Position Based Dynamics solver, distance constraints, and particle bodies.
- **locus (Parent B)**: Provides the non-Euclidean boundary wrapping (e.g., Torus, Klein Bottle, Mobius strip).
- **Novel Trait**: Rigid physics simulations that continuously loop and wrap through non-Euclidean coordinates. For instance, a chain of particles will fall off the bottom of the screen and seamlessly retain its rigid structure as it wraps around and falls from the top of the screen (or twists in the case of a Klein bottle).

## Run
```bash
cargo run -p physics-locus
```
