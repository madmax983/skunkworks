# Miller Reaction

**Parents**: `experiments/miller-fs` + `experiments/reaction-monitor`

## Concept
A "Living Crystal Filesystem". The file system structure is visualized as a 3D crystal lattice (using Miller indices for growth direction), where the surface texture of every crystal face displays a living Reaction-Diffusion simulation (Gray-Scott model).

The simulation parameters (Feed/Kill rates) are driven by the system's real-time load (CPU/RAM usage), creating a feedback loop where the computer's effort to render the visualization changes the visualization itself.

## Phenotype
- **Structure**: 3D Crystal Lattice (from `miller-fs`).
- **Texture**: Gray-Scott Reaction Diffusion (from `reaction-monitor`).
- **Dynamics**:
  - Crystals grow based on directory depth/content.
  - Surface patterns evolve organically.
  - High CPU activity alters the "Feed" rate, changing pattern stability.
  - High RAM usage alters the "Kill" rate.

## Controls
- **W/A/S/D**: Move camera.
- **Space/Shift**: Up/Down.
- **U/I/J/K/N/M**: Adjust Miller Indices (Plane visualization).

## Lineage
- `miller-fs`: Provided the WGPU instance rendering and crystallography logic.
- `reaction-monitor`: Provided the Compute Shader implementation of the Gray-Scott model and system monitoring logic.
- **Hybridization**: The compute shader output is used as a texture for the crystal instances in the render pass.
