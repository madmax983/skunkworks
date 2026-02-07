# Fertile Soil

> "The land is not a static stage, but a slow, churning ocean." - Genesis (The Geologist)

A reaction-diffusion simulation where the concentration of chemicals determines the elevation and biome of a generated terrain.

## Controls

- **Arrow Up/Down**: Adjust Feed Rate ($f$).
- **Arrow Left/Right**: Adjust Kill Rate ($k$).
- **R**: Reset Simulation.
- **Left Click**: Plant Catalysts (Add chemical B).

## Architecture

- **Engine**: `macroquad`.
- **Physics**: Gray-Scott Model implemented in GLSL fragment shader (Ping-Pong technique).
- **Visualization**: Terrain shader with derivative-based normal mapping and height-based biome coloring.
- **Audio**: Procedural drone synthesis modulated by simulation parameters (feed/kill rates) using `rodio` (optional feature).

## Running

```bash
cargo run -p fertile-soil --features audio
```
