# Black Hole Typewriter ⚛️🔭🕳️

> "The alphabet is the event horizon of language."

**Black Hole Typewriter** is a gravitational lensing text editor. It simulates a massive singularity that distorts the space-time of the canvas, bending the light of the characters you type.

The characters themselves are massive bodies, subject to N-body gravitational attraction and the overwhelming pull of the central singularity (your mouse cursor).

## Controls

-   **Type**: Spawn new characters at random locations with random velocities. Each character adds mass to the universe.
-   **Mouse**: Control the position of the Singularity. Drag it around to lens the text and pull characters into orbit (or spaghettification).
-   **Visuals**: The background is distorted by a custom GLSL fragment shader approximating Schwarzschild gravitational lensing.

## Tech Stack

-   **Engine**: `macroquad`
-   **Physics**: Symplectic Euler Integration, N-Body Gravity ($O(N^2)$), Softened Gravitational Potential.
-   **Shaders**: GLSL fragment shader for post-processing lensing effect.

## Build

```bash
cargo run --release --bin black-hole-typewriter
```

## Concept

This experiment combines **Gravitational Lensing** with **Text Distortion Effects**. It explores the idea of "Weighty Words"—where information has mass and gravity influences legibility.
