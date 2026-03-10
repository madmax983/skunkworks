# Origami Constellation ⚛️🦢

**Genesis Experiment**
Moonshot: Rigid Origami + Deployable Structure Simulation.

A simulation of a Miura-ori tessellation that deploys like a solar array or a space telescope mirror.
The vertices are visualized as stars in a constellation, and the edges are the connections.

## Controls

*   **Left/Right Arrow**: Fold/Unfold the structure.
*   **Mouse Drag (Left)**: Orbit camera.
*   **Mouse Drag (Right)**: Fold/Unfold (alternative).
*   **Scroll**: Zoom in/out.

## Implementation Details

*   **Math**: Parametric derivation of the Miura-ori fold based on a single extension factor.
*   **Visuals**: `macroquad` for 3D rendering with a "Deep Space" theme.
*   **Constraint**: The simulation maintains exact rigid body constraints for the faces (parallelograms) throughout the folding process.

## Concept

Mapping the mechanical act of deployment to the birth of a constellation.
As the structure unfolds, the stars align into a grid.
Stowed state = Dense cluster (Nebula).
Deployed state = Structured grid (Constellation).
