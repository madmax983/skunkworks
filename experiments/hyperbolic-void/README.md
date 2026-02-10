# Hyperbolic Void ⚛️🍩

An explorer of 3D Hyperbolic Space using the Poincaré Ball model.
Navigating this space feels non-Euclidean: parallel lines diverge, and space expands exponentially.

## Controls
- **WASD**: Move (Lorentz Boosts)
- **Q/E**: Rotate (Euclidean Rotation at origin)

## Tech Stack
- **Rust** 🦀
- **wgpu** (WebGPU)
- **cgmath** (Matrix math)
- Custom Vertex Shader for Hyperboloid -> Poincaré Ball projection.

## Topology
The world consists of a "Hyperbolic Cube".
As you move, the cube distorts because you are seeing it from different reference frames in the Hyperboloid model.
The "Fog" represents the boundary of the Poincaré Ball (Infinity).
