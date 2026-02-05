# Quasicrystal Dungeon

**"Forbidden symmetries frozen in GPU memory."**

A 3D interactive explorer for Icosahedral Quasicrystals, generated via 6D-to-3D projection (cut-and-project method). Rendered with `wgpu` using instanced rendering for millions of "atoms".

## Concept
Quasicrystals exhibit long-range order but no translational symmetry. The icosahedral quasicrystal has 5-fold, 3-fold, and 2-fold rotational symmetries, forbidden in periodic crystals. This experiment visualizes this structure as a "dungeon" of atoms—points in a high-dimensional lattice projected into our reality.

## Controls
- **W/S**: Move Forward/Back
- **A/D**: Move Left/Right
- **Space/Shift**: Move Up/Down
- **Esc**: Exit

## Usage
Run interactive window:
```bash
cargo run -p quasicrystal-dungeon
```

Run headless (generate `output.png`):
```bash
cargo run -p quasicrystal-dungeon -- --headless
```

## Tech Stack
- **wgpu**: WebGPU-based rendering (Metal/Vulkan/DX12).
- **cgmath**: Linear algebra.
- **winit**: Window handling.
- **bytemuck**: Zero-copy casting.
- **pollster**: Async executor for wgpu initialization.
