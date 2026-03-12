# Genesis: Tesseract Time ⚛️⬛

**A 4D Visualization Experiment**

Part of the "Spaces that Shouldn't Exist" series.

## Concept
This experiment visualizes a 4D chaotic attractor (specifically a Lissajous Knot) embedded within a rotating Tesseract (Hypercube).
It projects 4D coordinates $(x, y, z, w)$ onto 3D space using perspective projection from the 4th dimension.

The "Time Series" is represented by the trail of a particle moving through 4D space, leaving a glowing history.

## Controls
- **Arrow Keys**: Rotate the 3D Camera around the Tesseract.
- **W / S**: Zoom in/out (move camera distance).
- **Q / E**: Rotate the Tesseract in the XW plane (4D rotation).
- **R / F**: Rotate the Tesseract in the YW plane.
- **Space**: Toggle automatic 4D rotation.

## Implementation Details
- **Math**: Custom `Vec4` struct with 4D rotation matrices and perspective projection.
- **Geometry**: Procedurally generated Tesseract edges (32 edges connecting 16 vertices).
- **Attractor**: A 4D Lissajous curve with prime frequencies to create a non-repeating knot structure.
- **Rendering**: `macroquad` immediate mode 3D lines.

## Why?
Because looking at a 3D shadow of a 4D object spinning in 4D makes your brain hurt in a good way.
