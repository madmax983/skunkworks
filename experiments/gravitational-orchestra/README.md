# Gravitational Orchestra ⚛️🔭🎵📜

> "In the beginning was the Word, and the Word was with Gravity." — Genesis

**Gravitational Orchestra** is a Moonshot experiment combining **Gravitational Lensing** visualization with **Orbital Resonance** sonification.

## Concept

Massive bodies distort spacetime, bending the light of the background "Cosmic Code".
As these bodies orbit, their velocities and positions drive a generative audio engine, creating a "Music of the Spheres" where harmony emerges from stable orbits and dissonance from chaos.

## Features

-   **N-Body Physics**: Symplectic Verlet integration for stable orbits.
-   **Gravitational Lensing Shader**: Real-time ray deflection of a background text field.
-   **Generative Audio**: Sine wave synthesis based on orbital velocity and mass.
-   **Interactive**: Add bodies, reset the universe, and watch the text warp.

## Controls

-   **Left Click**: Spawn a new massive body with random velocity.
-   **Right Click**: Remove the last spawned body.
-   **Space**: Reset the simulation to the initial central star.

## Tech Stack

-   **Visuals**: `macroquad` + GLSL Fragment Shader.
-   **Audio**: `cpal` (real-time synthesis).
-   **Physics**: Custom N-Body engine.

## Running

```bash
cargo run -p gravitational-orchestra
```

*Note: Requires `libasound2-dev` on Linux for audio support.*
