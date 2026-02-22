# Hyper Acoustics

**A 4D Acoustic Simulation Experiment**

> "Hearing the shape of hyperspace."

## Concept

This experiment simulates the propagation of sound waves in a 4-dimensional hypercube. The simulation solves the 4D Wave Equation using a Finite Difference Time Domain (FDTD) method on a discrete grid.

The simulation is visualized by projecting the 4D pressure field into 3D space, where each cell is rendered as a particle that glows based on its acoustic energy.

## Lineage

*   **Parent A**: `experiments/tesseract-ops` (4D Hypercube Geometry, System Monitor)
*   **Parent B**: `experiments/resonant-chamber` (2D FDTD Wave Physics)
*   **Novel Trait**: 4D Sound Propagation. The speed of sound and damping are modulated by real-time system metrics (CPU/RAM).

## Features

*   **4D FDTD Solver**: Solves `d2u/dt2 = c^2 * Laplacian(u)` in 4 dimensions.
*   **System Modulation**:
    *   **CPU Load** -> Wave Speed ($c^2$). Higher load makes waves travel faster (or more chaotically).
    *   **RAM Usage** -> Damping. Higher memory usage increases viscosity/damping.
    *   **Swap Usage** -> Random Plucks (Entropy/Noise).
*   **Visualization**:
    *   Renders a 3D projection of the 4D grid.
    *   Particles glow Red/Blue based on pressure (+/-).
    *   The Tesseract rotates in 4D space (ZW rotation added).

## Controls

*   **Arrow Keys**: Rotate Camera around the 3D projection.
*   **W/S**: Zoom In/Out.
*   **Left Click**: Pluck a random point in the 4D grid.
*   **Space**: (Not implemented yet)

## Building

This experiment uses `cpal` for audio output.

*   **With Audio**: `cargo run --features audio` (Requires ALSA/Jack/PulseAudio headers).
*   **Simulation Only**: `cargo run` (Default). Runs the physics and visualization without audio output.
