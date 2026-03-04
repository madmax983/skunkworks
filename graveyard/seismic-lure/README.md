# Seismic Lure 🌊🌋

**Lineage**: `fissure-tracker` × `cymatic-lure`

## Concept

**Seismic Lure** sonifies the technical debt of a codebase. It combines the structural analysis of `fissure-tracker` with the wave simulation and audio synthesis of `cymatic-lure`.

Your code files are nodes in a force-directed graph. Files with high "stress" (TODOs, unwraps, panics) act as active geological faults. They sit on top of a virtual wave tank.

When the simulation runs, these stressed nodes vibrate, disturbing the water surface. The resulting wave interference patterns are scanned to generate a drone soundscape unique to your codebase.

## Lineage

*   **From `fissure-tracker`**:
    *   File scanning logic (counting keywords).
    *   Force-directed graph physics (nodes repelling).
*   **From `cymatic-lure`**:
    *   FDTD Wave Equation simulation.
    *   Scanned Synthesis audio engine.
    *   TUI height map rendering.
*   **Novel Trait**:
    *   **Geological Sonification**: Visualizing and hearing code quality as a physical interaction between structure (files) and medium (waves).

## Controls

*   **WASD**: Pan the camera.
*   **+/-**: Zoom in/out.
*   **Up/Down**: Adjust base frequency.
*   **Q / Esc**: Quit.

## Usage

```bash
cargo run -p seismic-lure -- [path_to_scan]
```

Defaults to current directory if no path is provided.

## Legend

*   **● (Red)**: High Stress (Many issues).
*   **● (Yellow)**: Moderate Stress.
*   **○ (Green)**: Stable.
*   **Blue/Cyan Background**: Wave height (Water).
