# 32. Extract Flocking Logic

Date: 2024-05-22

## Status

Accepted

## Context

The repository contains numerous experiments simulating flocking behavior (e.g., `hydro-boids`, `sono-boids`, `quantum-boids`). Each experiment historically implemented its own version of Reynolds' flocking rules (Separation, Alignment, Cohesion). This led to:

1.  **Code Duplication**: The core physics loop was copy-pasted across multiple directories.
2.  **Inconsistent Physics**: Slight variations in force application or integration methods (Euler vs Verlet) made it difficult to compare behaviors.
3.  **Maintenance Burden**: Fixing a bug in the separation logic required updates across N experiments.
4.  **Performance Optimization**: Improvements (e.g., spatial hashing) could not be easily shared.

## Decision

We have extracted the core flocking physics and state management into a dedicated library crate: `crates/flocking`.

This crate provides:
1.  `PhysicsState`: A struct encapsulating position, velocity, and acceleration, using `locus::Vec2` for vector math.
2.  `FlockingParams`: A configuration struct for tuning weights and radii.
3.  `compute_force`: A pure function that calculates the net force vector for a given agent based on its neighbors.

## Consequences

### Positive
*   **DRY Principle**: Flocking logic is defined once and reused.
*   **Consistency**: All consuming experiments now share the exact same physics engine.
*   **Focus**: Experiment code can focus on the unique aspect (e.g., sound generation in `sono-boids`) rather than reimplementing physics.
*   **Testability**: The physics logic is isolated and unit-tested within the crate.

### Negative
*   **Dependency**: Experiments now depend on `crates/flocking` (and transitive `crates/locus`).
*   **Rigidity**: Experiments needing radically different physics models (e.g., non-Newtonian movement) may find the shared struct limiting, though they can still opt-out.
