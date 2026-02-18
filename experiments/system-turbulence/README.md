# System Turbulence 🌊🖥️

A GPU-accelerated fluid simulation driven by your computer's internal state.
Every process is a particle in a turbulent sea. High CPU usage creates vortices. Memory leaks add mass.

![System Turbulence](https://raw.githubusercontent.com/jules-ai/system-turbulence/main/screenshot.png)

## Concept

"What if the Operating System was a fluid?"

This experiment maps system metrics (CPU usage, Memory usage, Process IDs) to a 2D Navier-Stokes fluid simulation running entirely on the GPU using `wgpu` compute shaders.

-   **Density**: Visual representation of system activity.
-   **Velocity**: Driven by process "heat" and "movement".
-   **Interaction**: Your mouse stirs the soup.

## Tech Stack

-   **Rust**: For performance and type safety.
-   **Bevy**: ECS and Render Graph management.
-   **wgpu**: Direct Compute Shader (WGSL) implementation.
-   **sysinfo**: Real-time system monitoring.

## How to Run

```bash
cargo run --release
```

## Controls

-   **Mouse**: Stir the fluid.
-   **R**: Reset simulation.

## Architecture

1.  **System Monitor**: A background thread polls `sysinfo` and updates a shared `SystemStats` resource.
2.  **Force Injection**: A Bevy system maps process stats to a "Force Texture" (RGBA32Float) on the CPU.
3.  **Compute Pass**: A custom Render Graph Node dispatches a compute shader that:
    -   Advects density and velocity.
    -   Injects forces from the texture.
    -   Applies diffusion and decay.
4.  **Visualization**: The simulation texture is rendered to a full-screen quad using a custom color mapping shader.

## License

MIT / Apache 2.0
