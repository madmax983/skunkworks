# Harmony of Spheres ⚛️🔭🎵

> "There is geometry in the humming of the strings, there is music in the spacing of the spheres." — Pythagoras

**Harmony of Spheres** is an N-body gravitational simulation that acts as a real-time additive synthesizer. Each celestial body creates a tone based on its velocity and distance from the observer.

When planets fall into stable orbits, they form repeating musical intervals. When the system is chaotic, the soundscape is dissonant and evolving.

## Controls

-   **Arrow Keys**: Pan the camera.
-   **+/-**: Zoom in/out.
-   **Click (Left)**: Spawn a new planet at the mouse position. The initial velocity is calculated for a stable circular orbit relative to the central star (if the system was simple), but N-body interactions will quickly perturb it.
-   **Space**: Pause/Resume simulation.
-   **R**: Reset the universe to the initial 3-planet configuration.
-   **C**: Clear orbital trails.

## Audio Synthesis

The audio engine uses `cpal` to generate sound in real-time.
-   **Frequency (Pitch)**: Determined by the scalar velocity of the body. Faster = Higher Pitch.
-   **Amplitude (Volume)**: Determined by proximity to the central star/observer. $A \propto 1/r$.

*Note: If compiled without the `audio` feature (default in some environments), the simulation runs in "Silent Mode".*

## Tech Stack

-   **Visuals**: `macroquad`
-   **Physics**: Symplectic Euler Integration for energy stability.
-   **Audio**: `cpal` (gated behind `audio` feature).

## Build Instructions

To build with audio enabled:

```bash
cargo run --release --features audio
```

To build in silent mode (default):

```bash
cargo run --release
```
