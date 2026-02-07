# Morpho-Synth ⚛️🎹

A reaction-diffusion sequencer where fluid dynamics create music.

## Concept
Based on the Gray-Scott model, `Morpho-Synth` simulates chemical morphogens (U and V) diffusing and reacting on a 2D grid. The concentration of chemical V drives an additive audio synthesis engine.

## Usage
- **Left Click**: Add "Feed" (chemical V) to the system. Create life.
- **Right Click**: Place a "Sensor". Sensors play a pentatonic note when the local concentration of V exceeds a threshold.
- **Arrow Keys**: Adjust Feed and Kill rates (Up/Down for Feed, Left/Right for Kill).

## Tech Stack
- **Simulation**: `rayon` parallelized CPU solver (9-point Laplacian stencil).
- **Visuals**: `macroquad` rendering.
- **Audio**: `cpal` real-time synthesis (additive sine waves).
- **Novelty**: Combining fluid simulation with musical sequencing.
