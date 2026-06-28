# 118. Flock-Physics Hybrid

Date: 2026-06-15

## Status

Accepted

## Context

The experimental `flock-physics` hybrid aims to combine emergent behavioral intelligence with rigid, constraint-based structural integrity. We crossed `crates/flocking` (simulating boids with separation, alignment, and cohesion) with `crates/physics-pbd` (Position Based Dynamics, simulating soft-body constraints).

## Decision

We mapped the boids from the flocking simulation onto the constraint nodes of a soft-body mesh simulated by Position Based Dynamics. The swarm intelligence drives the kinetic movement of the particles, while the PBD system strictly enforces distance constraints, effectively making the swarm act as a single, cohesive soft-body organism deforming in real-time.

## Consequences

- **Positive:** Yields highly organic, amoeba-like movements. The "boids" pull the soft-body mesh in various directions based on flocking rules, while the PBD constraints prevent the mesh from tearing apart.
- **Negative:** The two physics systems exist in constant tension. Careful tuning of flocking weights versus PBD constraint stiffness is required to prevent the simulation from instantly exploding or completely freezing.
