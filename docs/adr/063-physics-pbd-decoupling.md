# 063. Decouple Physics PBD from Macroquad

Date: 2024-05-27

## Status

Accepted

## Context

The `physics-pbd` crate implements a Position Based Dynamics engine used by experiments such as `neuro-fold`, `neuro-cipher`, and `origami-swarm` to simulate interconnected particles.

Initially, `physics-pbd` depended on the `macroquad` crate solely for its vector math (`Vec3` types). `macroquad` is a full-featured game engine. This created a significant "leak" where consumers of the physics engine were forced to pull in heavy graphics, windowing, and audio dependencies, even if they were headless servers or Terminal User Interfaces (TUIs).

## Decision

We have decoupled `physics-pbd` from rendering logic by removing the `macroquad` dependency.

1.  **Replaced `macroquad` with `glam`:** The `glam` crate provides the necessary SIMD-accelerated linear algebra types (`Vec3`, `Mat4`, etc.) without the weight of a game engine.
2.  **Updated `physics-pbd`:** All vector math within the solver now uses `glam`.

## Consequences

### Positive
*   **Decoupling:** Physics logic is strictly isolated from rendering.
*   **Portability:** The physics engine can now be used in TUI, server, or web (WASM without WebGL) contexts without pulling in unnecessary dependencies.
*   **Faster Builds:** Crates depending on `physics-pbd` will compile faster as they no longer need to compile `macroquad` and its system dependencies.

### Negative
*   **Type Conversion:** Experiments that *do* use `macroquad` for rendering may need to handle conversions between `glam` and `macroquad` vector types, though `macroquad` itself re-exports `glam`, mitigating this in most cases.
