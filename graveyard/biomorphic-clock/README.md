# Biomorphic Clock ⚛️🕰️

A chemical clock simulated using an anisotropic Gray-Scott Reaction-Diffusion system.

## Concept

Time is not just a number; it's a direction.
This simulation uses a Reaction-Diffusion system where the diffusion tensor rotates 360 degrees every minute.
- **Angle 0° (0s)**: Diffusion is horizontal.
- **Angle 90° (15s)**: Diffusion is vertical.
- **Angle 180° (30s)**: Diffusion is horizontal.
- **Angle 270° (45s)**: Diffusion is vertical.

The Turing patterns (spots/stripes) align with the diffusion direction, creating a "Texture of Time".

## Implementation

- **Language**: Rust
- **Simulation**: Gray-Scott Model with Anisotropic Laplacian.
- **Parallelism**: `rayon` for multi-threaded grid updates.
- **Visualization**: `ratatui` (TUI).

## Controls

- **Q**: Quit
- **R**: Reset and re-seed the reaction.

## How to Run

```bash
cargo run --release
```

## Theory

The standard Gray-Scott model uses isotropic diffusion:
$$ \frac{\partial u}{\partial t} = D_u \nabla^2 u ... $$

Here, we use a rotated Gaussian kernel to bias the diffusion:
$$ K(\theta) \approx \exp \left( - \frac{x'^2}{2\sigma_{long}^2} - \frac{y'^2}{2\sigma_{short}^2} \right) $$
where $x', y'$ are coordinates rotated by the current time angle.

This breaks the symmetry of the reaction, forcing the emergent structures to align with the "Time Field".
