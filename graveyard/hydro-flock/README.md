# Hydro Flock 🐟🔥

**The Splice Surgeon 🧬**

> "Life thrives on the gradient."

A hybrid of **Hydrothermal Locks** and **Luminous Flock**.

## Lineage
- **Parent A**: `experiments/hydrothermal-locks` (Fluid dynamics, Heat, Vents)
- **Parent B**: `experiments/luminous-flock` (Boid Flocking)

## Concept

This experiment visualizes a swarm of "Thermophilic Boids" (Heat-seeking flocking agents) inhabiting a hydrothermal vent field.

- **Vents (Locks)** emit Heat into the fluid simulation.
- **Boids** flock together using Separation, Alignment, and Cohesion rules.
- **Thermal Taxis**: Boids are attracted to heat sources (Vents).
- **Hydrodynamics**: Boids are pushed by buoyant currents (Heat rises).
- **Contention**: When boids crowd a vent, they generate *more* heat, creating a feedback loop.

## The Simulation

1.  **Fluid**: Simulates heat diffusion and convection.
2.  **Boids**:
    - Yellow triangles.
    - Flock together.
    - Seek heat.
    - Are pushed by currents.
3.  **Particles**: Smoke/Sediment spawns from hot areas.
4.  **Worms**: Tube worms (legacy from Parent A) grow on the chimneys.

## Running

```bash
cargo run -p hydro-flock
```
