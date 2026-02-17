# Quasicrystal Dungeon

**"Forbidden symmetries frozen in GPU memory."**

A 3D interactive explorer for Icosahedral Quasicrystals, generated via 6D-to-3D projection (cut-and-project method). Rendered with `wgpu` using instanced rendering for millions of "atoms".

## The Aperiodic Roguelike
This experiment combines mathematical crystal generation with procedural dungeon mechanics.
The dungeon layout is determined by the connectivity of the quasicrystal lattice:
- **Hubs (High Connectivity)**: Likely to contain **Treasure** (Gold) or **Bosses** (Purple).
- **Corridors (Low Connectivity)**: Likely to contain **Enemies** (Red) or **Traps** (Gray).
- **Goal**: Find the furthest node from the start (Red marker).

## Controls
- **W/S**: Move Camera Forward/Back
- **A/D**: Move Camera Left/Right
- **Space/Shift**: Move Camera Up/Down
- **Tab**: Cycle available neighbors to move to (Yellow highlight).
- **Enter**: Move Player (White) to selected neighbor.
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
- **rand**: Procedural generation.
