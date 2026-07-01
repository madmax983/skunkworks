# 129. Git-Physics Hybrid

Date: 2026-06-23

## Status

Accepted

## Context

The Splice Surgeon created a new experimental hybrid called `git-physics`. This experiment crossbreeds the discrete, chronological commit history of a repository provided by `crates/git-associates` with the continuous Position-Based Dynamics rigid body simulation from `crates/physics-pbd`.

## Decision

We map discrete repository modifications directly to physical particles dropping into a rigid body simulation. Each commit spawns as a physical body whose mass and size scale proportionally with its codebase impact (the total number of insertions and deletions). These commit-particles are subject to gravitational forces and undergo continuous physical collision detection as they settle.

## Consequences

- **Positive:** Produces an intuitive, emergent visualizer where the accumulated weight and evolution history of a codebase materialize as a physical pile of interacting, colliding bodies, translating abstract repository history into a visceral, kinetic environment.
- **Negative:** Simulating a very large number of commits, particularly massive ones, can lead to physics constraint instability or particle clipping (requiring arbitrary manual bounding constraints and limiting the history length to maintain performance).
