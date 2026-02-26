# 59. Consolidate Geometry and Flocking in Locus

Date: 2024-05-25

## Status

Accepted

## Context

The codebase suffered from fragmentation in its core mathematical types and spatial logic:

1.  **Vector Fragmentation**: `Vec2` resided in `locus`, while `Vec3` and `Vec4` were located in `hyper-system`. This forced crates needing 3D logic to depend on a "hyper" system crate intended for 4D visualization, introducing unnecessary bloat and semantic confusion.
2.  **Flocking Isolation**: The `flocking` crate implemented Reynolds' boid rules but depended on `locus` for `Vec2`. However, the logic for spatial separation and alignment is fundamental to many simulations, not just "flocking" experiments. Keeping it separate created circular dependency risks if `locus` ever needed spatial partitioning.
3.  **Type Conversion Overhead**: Moving data between 2D, 3D, and 4D contexts often required manual conversion or bridging traits because the types lived in different crates.

## Decision

We will consolidate all geometry and spatial behavior logic into the `locus` crate.

1.  **Move `Vec3` and `Vec4` to `locus`**: The `locus` crate will become the canonical source for all linear algebra types (`Vec2`, `Vec3`, `Vec4`).
2.  **Merge `flocking` into `locus`**: The standalone `flocking` crate will be dissolved. Its logic will move to `locus::flocking`, making spatial behavior a core feature of the geometry library.
3.  **Repurpose `hyper-system`**: The `hyper-system` crate will retain its specific domain logic (system monitoring, 4D rendering utilities) but will re-export vector types from `locus` instead of defining them.

## Consequences

*   **Positive**:
    *   **Single Source of Truth**: Any experiment needing vector math only needs to depend on `locus`.
    *   **Simplified Dependency Graph**: Removes the `flocking` node and reduces `hyper-system` to a leaf node or higher-level utility.
    *   **Enhanced Capability**: `locus` becomes a more robust "standard library" for spatial computing in the Skunkworks.

*   **Negative**:
    *   **Larger Locus Crate**: The `locus` crate grows in size, potentially increasing compile times for small experiments that only need `Vec2`. (Mitigated by feature flags if necessary, though currently acceptable).
