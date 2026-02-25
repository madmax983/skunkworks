# Hyper Tissue 🧬🧊

**Status**: Experimental
**Lineage**: `chimera-tissue` × `tesseract-ops`

A 4D Soft-Body Simulation where the "muscle fibers" are driven by genetic code (ChimeraVM) and the entire universe breathes based on the computer's CPU load.

## Concept

This experiment simulates a "Hyper-Tissue" — a 3Dx3D grid of particles existing in 4D space (a Tesseract structure).

- **4D Physics**: A custom Position Based Dynamics (PBD) solver operates on 4D vectors (`Vec4`). Particles move and interact in X, Y, Z, and W dimensions.
- **Chimera Muscles**: Each cell (particle) contains a ChimeraVM instance.
    - **Input**: Local 4D Strain (average extension of connected constraints).
    - **Input**: Global System CPU Load (injected as a hormone).
    - **Output**: Contraction Factor (modulates the rest length of 4D actuator constraints).
- **Hyper-Elasticity**: The tissue can contract in the 4th dimension to avoid 3D obstacles or to "hide".
- **System Distortion**: The W-axis scaling and rotation are influenced by the host machine's real-time performance metrics.

## Controls

- **Left Click + Drag**: Rotate the 3D projection camera.
- **Automatic**: The 4D rotation (XW plane) happens automatically, revealing the hyper-structure.

## Implementation Details

- **Physics**: Implements a `PbdSystem4D` solver in `src/physics.rs` using `hyper_system::math::Vec4`.
- **Rendering**: Projects 4D coordinates to 3D using a stereographic projection (`project_to_3d`), then renders using Macroquad's 3D drawing functions.
- **Genetics**: Uses `chimera-lang` to define a simple "Oscillator DNA" that reacts to strain and CPU load.

## Lineage

- **Parent A**: `chimera-tissue` (Soft body mechanics, genetic actuators).
- **Parent B**: `tesseract-ops` (4D visualization, system monitoring).
- **Novel Trait**: 4D Bio-Mechanics. Muscles that pull across the 4th dimension.

## The Splice Surgeon's Note

"The tissue breathes. Not just in and out, but *across*. When the CPU spikes, the hypercube shudders."
