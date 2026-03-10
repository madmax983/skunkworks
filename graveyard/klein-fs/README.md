# Klein FS ⚛️🍩

> "The files are inside... but there is no inside."

A 3D file system explorer mapped onto a **Figure-8 Klein Bottle**.

## Concept
The file system is treated as a single continuous stream of data. This stream is wrapped around a non-orientable surface (Klein Bottle) to create an "infinite loop" interface.
- **Surface**: Figure-8 Immersion of the Klein Bottle.
- **Mapping**: Directory traversal is flattened into a linear spiral path `(u, v)` on the surface.
- **Infinite Scroll**: Navigation wraps around the bottle seamlessly.

## Controls
- **Scroll Wheel**: Navigate forward/backward through the file list.
- **Drag (Left Click)**: Orbit camera around the current focus point.
- **W / S**: Zoom in/out.

## Math
Parametric equations for the Figure-8 Immersion:
```math
x = (r + cos(u/2)sin(v) - sin(u/2)sin(2v)) cos(u)
y = (r + cos(u/2)sin(v) - sin(u/2)sin(2v)) sin(u)
z = sin(u/2)sin(v) + cos(u/2)sin(2v)
```
Where `u` represents the file index (spiral progress) and `v` represents the "loop" parameter.

## Tech Stack
- **Rust**
- **Macroquad**
- **Walkdir**
