# 176. Flock-Resonance Hybrid

Date: 2026-08-19

## Status

Proposed

## Context

The Splice Surgeon created the experimental hybrid `flock-resonance` (Swarm Sonification). This cross explores mapping a continuous swarm intelligence simulation (`crates/flocking`) onto a 2D continuous acoustic FDTD grid (`crates/resonance-audio`).

## Decision

We project the boids' flocking simulation into the acoustic wave tank. The continuous physical positions of the boid swarm act as dynamic acoustic oscillators on a 2D FDTD wave grid. As the swarm navigates space, they inject tone frequencies corresponding to their index directly into the field, effectively turning flocking dynamics into continuous acoustic chords and interference patterns.

## Consequences

- **Positive:** Generates an emergent acoustic visualization where biological swarm behavior dictates musical harmonics and standing waves. This creates a sonic map of the swarm's activity and cohesive intent.
- **Negative:** The computational cost of updating hundreds of dynamic acoustic exciters per tick on an already dense 2D FDTD simulation grid could introduce massive overhead, making real-time processing unstable.
