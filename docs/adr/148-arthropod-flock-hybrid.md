# 148. Arthropod-Flock Hybrid

Date: 2026-07-06

## Status

Accepted

## Context

The Splice Surgeon created a new experimental hybrid called `arthropod-flock` (Interactive Swarm Intelligence). This cross explores mapping by crossing the discrete components of `crates/arthropod` with `crates/flocking`.

## Decision

Cross the immediate mode UI elements of `arthropod` with the swarm intelligence of `flocking`. The GUI dynamically maps to the weights of the `FlockingParams`.

## Consequences

- **Positive:** Allows real-time interactive manipulation of the boid DNA, shifting their rules natively via GUI.
- **Negative:** An emergent interactive sandbox where interface buttons cause the chaotic biological swarm to scatter, align, or group tightly.
