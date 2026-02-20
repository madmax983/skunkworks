# 45. Standardize GPU Compute Architecture

Date: 2024-05-23

## Status

Accepted

## Context

Several experiments in the repository, such as `ising-tide` (Ising Model), `miller-fs` (Lattice Filesystem), `crystal-defense` (Quasicrystal Projection), and `fluid-rain` (Fluid Dynamics), require high-performance cellular automata or physics simulations. These simulations often involve grid sizes of $256^3$ or more, resulting in millions of cells or agents.

Traditional CPU-based iteration (even with `rayon` parallelism) becomes a bottleneck at this scale, preventing real-time interaction (60 FPS). Additionally, transferring state between CPU and GPU for rendering adds significant latency.

We needed a standardized approach to handle these high-load simulations while maintaining code clarity and reusability across experiments.

## Decision

We will adopt **`wgpu` Compute Shaders** as the standard mechanism for high-performance lattice and particle simulations.

1.  **Compute Pipelines:** Logic for cell updates (physics, automata rules) will be written in WGSL and executed on the GPU via `wgpu::ComputePipeline`.
2.  **Ping-Pong Buffering:** To avoid race conditions and memory hazards during parallel updates, we will utilize a double-buffering strategy (`[wgpu::Buffer; 2]`). One buffer acts as the read-only source ("Ping"), and the other as the write-only destination ("Pong"). The indices are swapped each frame.
3.  **Instanced Rendering:** The simulation state will remain on the GPU. We will use `wgpu::RenderPipeline` with instanced rendering (or vertex pulling) to visualize the data directly from the storage buffers, eliminating the need for CPU readback.

## Consequences

### Positive
*   **Massive Performance:** Enables real-time simulation of millions of entities, far exceeding CPU capabilities.
*   **Decoupling:** Simulation logic is isolated in shaders, freeing up the CPU for UI (TUI/GUI), input handling, and orchestration.
*   **Visualization:** Zero-copy rendering allows for complex visualizations (volumetric, instanced meshes) without bus overhead.

### Negative
*   **Boilerplate:** Setting up `wgpu` (Device, Queue, Surface, Pipelines, BindGroups) requires significantly more code than a simple `Vec<Cell>`.
*   **Debugging:** Debugging WGSL shaders is more difficult than debugging Rust code (no `println!`, limited tooling).
*   **Platform Constraints:** Web deployment requires WebGPU support, which is not yet universally available, necessitating fallbacks or limiting the audience.
*   **Complexity:** The "Ping-Pong" logic adds state management complexity (tracking frame counts and buffer indices).

## References

*   `experiments/ising-tide`: Canonical implementation of 3D Ising Model using this pattern.
*   `experiments/miller-fs`: Adapts the pattern for a filesystem visualization lattice.
*   `experiments/crystal-defense`: Uses compute shaders for projecting 6D quasicrystals.
