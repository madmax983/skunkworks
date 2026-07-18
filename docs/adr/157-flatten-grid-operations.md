# 157. Flatten Grid Operations

Date: 2026-07-14

## Status

Accepted

## Context

The `GridOps` module in `chimera-lang/src/vm/ops/grid.rs` executed all grid-related operations (`GRead`, `GWrite`, `Radiate`, `Siphon`, `Virus`) inside a monolithic switch block within `exec_grid_op`. This violated high cohesion, leading to 'Pyramid of Doom' structures and making the module exceedingly difficult to read and test.

## Decision

Extract the logic for each grid operation into independent helper functions (`apply_g_read`, `apply_g_write`, `apply_radiate`, `apply_siphon`, `apply_virus`). Flatten control flow within these helpers by utilizing guard clauses and early returns for error handling (e.g., stack underflows, type mismatches, out-of-bounds errors).

## Consequences

- **Positive:** Greatly improves readability and maintainability of grid execution logic.
- **Positive:** Enables targeted unit testing for discrete grid operations without setting up the full VM execution state.
- **Negative:** Marginally increases the internal surface area of `ChimeraVM` with new methods.
