# 🧬 Folded Colony

**Hybrid:** `rigid-origami` × `biomimetic-bridge`

A simulation of an ant colony living on a dynamic Miura-ori folding surface.

## Concept

The ants inhabit a 2D manifold (the paper sheet), but their movement is influenced by the 3D topology of the fold.
As the sheet folds (User controls: Left/Right Arrows), points that are distant on the 2D manifold may become close in 3D Euclidean space.

**Novel Trait: Topology Jumping (Wormholes)**
Ants can detect these "wormholes" created by the folding geometry and jump across them, effectively teleporting across the 2D grid by traversing the 3D shortcut.

## Controls

- **Left/Right Arrows**: Fold/Unfold the Miura-ori sheet.
- **Mouse Drag**: Orbit camera.
- **Mouse Wheel**: Zoom.

## Lineage

- **Parent A (`rigid-origami`)**: Provided the kinematic equations for the Miura-ori tessellation and mesh generation.
- **Parent B (`biomimetic-bridge`)**: Provided the ant agent logic (foraging, pheromones).
- **The Splice Surgeon**: Implemented the "Wormhole" detection logic where ants scan 3D neighbors to find shortcuts.

## Technical Details

- Uses `macroquad` for 3D rendering.
- `MiuraGrid` calculates vertices analytically based on expansion factor.
- `Colony` updates ants using bilinear interpolation to map 2D grid coordinates `(u,v)` to 3D space `(x,y,z)`.
