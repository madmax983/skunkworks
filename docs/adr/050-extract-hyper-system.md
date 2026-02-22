# 050. Extract Hyper System Logic

## Status
Accepted

## Context
The **Hyper Series** of experiments (`hyper-fluid`, `hyper-acoustics`, `hyper-glass`, `hyper-market`, etc.) all explore high-dimensional simulations modulated by system metrics. These experiments share a common need for:
1.  **4D Vector Mathematics:** Operations on 4D vectors (`Vec4`), including rotations and projections to 3D space.
2.  **System Monitoring:** Real-time monitoring of CPU, Memory, Swap, and Load Average, with smoothing logic to prevent jitter in the simulation parameters.

Previously, this logic was duplicated across each experiment, leading to:
*   **Maintenance Burden:** Bug fixes or improvements to the math or monitoring logic had to be applied to multiple files.
*   **Inconsistency:** Different experiments used slightly different smoothing factors or vector implementations.
*   **Dependency Management:** Updating `sysinfo` versions required changes in every experiment's `Cargo.toml`.

## Decision
We extract the shared logic into a dedicated library crate `crates/hyper-system`.

This crate exports:
*   `math::Vec4`: A struct for 4D vector operations, including `rotate_xw`, `project_to_3d`, and standard arithmetic.
*   `monitor::SystemMonitor`: A struct that wraps `sysinfo::System`, handles the refresh loop, and provides smoothed (interpolated) metrics for `cpu_usage`, `mem_usage`, `swap_usage`, and `load_avg`.

All "Hyper" series experiments will depend on `crates/hyper-system` instead of implementing this logic locally.

## Consequences

### Positive
*   **DRY (Don't Repeat Yourself):** Logic is defined in one place.
*   **Consistency:** All hyper-experiments will respond to system load in the exact same way, making comparisons more valid.
*   **Simplified Maintenance:** Updates to `sysinfo` or optimizations to 4D math only need to happen in one crate.

### Negative
*   **Coupling:** Experiments are now coupled to a shared crate version. Breaking changes in `hyper-system` will require updating all dependent experiments.
*   **Build Time:** Changing `hyper-system` triggers a rebuild of all dependent experiments in the workspace.
