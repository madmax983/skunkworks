# 50. Extract Hyper System Logic

Date: 2025-05-18

## Status

Accepted

## Context

The "Hyper" series of experiments (`hyper-acoustics`, `hyper-market`, `hyper-ferro`, etc.) all share a common need for 4-dimensional vector mathematics and real-time system monitoring.

Initially, this logic was either duplicated across experiments or implemented in ad-hoc ways, leading to:
1.  **Code Duplication:** The same `Vec4` struct and operations were redefined multiple times.
2.  **Inconsistent Behavior:** Different experiments handled 4D-to-3D projection slightly differently.
3.  **Maintenance Overhead:** Bug fixes or optimizations (e.g., in vector normalization) had to be applied in multiple places.
4.  **Integration Friction:** New experiments required boilerplate code to set up system monitoring for driving visualization parameters.

## Decision

We have extracted the shared 4D math and system monitoring logic into a dedicated crate: `crates/hyper-system`.

This crate provides:
1.  **`math` Module:** A `Vec4` struct with optimized operations for arithmetic, geometric transformations (rotations in XW, YW, ZW planes), and stereographic projection to 3D.
2.  **`monitor` Module:** A `SystemMonitor` struct that tracks CPU, memory, swap, and load average, providing smooth interpolation for visualization purposes.

Experiments in the "Hyper" series now depend on this crate via a workspace dependency.

## Consequences

### Positive
*   **Single Source of Truth:** All 4D math operations are defined in one place, ensuring consistency.
*   **Reduced Boilerplate:** New experiments can immediately access robust vector math and system metrics.
*   **Optimized Performance:** improvements to math operations benefit all consumers immediately.
*   **Simplified Visualization:** The `SystemMonitor` abstracts away the complexity of `sysinfo` and provides normalized, interpolated values ready for rendering.

### Negative
*   **Coupling:** "Hyper" experiments are now coupled to the `hyper-system` crate. Changes to the crate API may require updates across multiple experiments.
*   **Dependency Weight:** Including `sysinfo` (via `hyper-system`) adds to the compile time and binary size of experiments, even if they only need the math module (though this can be mitigated with feature flags in the future).
