# 15. Extract Poincaré Disk Library

Date: 2024-10-25

## Status

Accepted

## Context

Several experiments in the Skunkworks repository explore hyperbolic geometry, specifically the Poincaré disk model. Examples include `hyperbolic-git` (visualizing git history), `chimera-lang` (mapping the grid to the disk), and `hyperbolic-fs` (filesystem navigation).

Previously, these experiments either implemented their own math logic for hyperbolic distance and Möbius transformations or shared source files in ad-hoc ways. This led to:
1.  **Code Duplication:** The same formulas for `mobius_add` or `hyperbolic_dist` appeared in multiple places.
2.  **Inconsistency:** Subtle bugs or differences in implementation (e.g., handling floating point singularities) could exist between experiments.
3.  **Testing Gaps:** Math logic embedded in UI code is harder to unit test effectively.

## Decision

We have extracted the core hyperbolic geometry logic into a dedicated shared crate: `crates/poincare-disk`.

This crate provides:
1.  `Point`: A type alias for `Complex<f64>` representing a point in the unit disk.
2.  `Mobius`: A struct representing Möbius transformations for isometry.
3.  `TilingConsts`: Precomputed constants for generating hyperbolic tilings (e.g., {4,5}).
4.  Helper functions: `mobius_add`, `mobius_sub`, `hyperbolic_dist`, `neighbor_transform_a`.

Consumers (experiments) must depend on this crate via the workspace or path dependency.

## Consequences

**Positive:**
-   **Centralization:** A single source of truth for hyperbolic math.
-   **Testability:** The crate has its own unit tests (including edge cases like singularities) independent of any TUI or graphical context.
-   **Reusability:** New experiments can immediately access robust geometry primitives.

**Negative:**
-   **Dependency Management:** Consumers must include `poincare-disk` and often `num-complex` in their `Cargo.toml`.
-   **Versioning:** Changes to the math library affect all dependent experiments, potentially requiring widespread updates if the API changes.
