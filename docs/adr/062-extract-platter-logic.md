# 062. Extract Platter Field Logic

## Status
Accepted

## Context
Multiple experiments in the "Ferrous" series (`ferrous-fluid`, `ferrous-chimera`, `ferrous-core`) require a shared 2D grid structure for simulating scalar fields. These fields represent various physical properties such as magnetism, fluid density, or pheromone trails.

Each experiment initially implemented its own version of a grid, leading to:
1.  **Code Duplication:** The core logic for grid storage, access, and modification was repeated.
2.  **Inconsistent Behavior:** Different implementations handled edge cases (like saturation limits or decay thresholds) differently.
3.  **Maintenance Overhead:** Optimization or bug fixes had to be applied in multiple places.

## Decision
We have extracted the `Platter` struct and its associated logic into a dedicated workspace library: `crates/platter`.

The `Platter` provides a standardized API for 2D scalar fields with two distinct update modes:
1.  **Magnetization (Saturation):** Adds values with a hard cap (e.g., 1.0), suitable for fields with physical limits.
2.  **Accumulation (Density):** Adds values without a cap, suitable for conserving quantities like fluid mass.

It also includes a uniform `decay` method to simulate dissipation over time, with specific handling for small values to prevent denormal number performance penalties.

## Consequences

### Positive
*   **Reuse:** New experiments can immediately leverage a robust field simulation primitive.
*   **Consistency:** All simulations using `Platter` will share the same behavior for saturation and decay.
*   **Performance:** Optimizations (like the decay threshold check) are centralized.

### Negative
*   **Coupling:** Changes to `crates/platter` will require updates to all dependent experiments.
