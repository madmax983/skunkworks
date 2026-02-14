# Klein Files ⚛️🍩

**"Topology is geometry without measurement—only connection matters."**

A file system visualizer that maps your directory structure onto the surface of a **Figure-8 Klein Bottle**.
This is a non-orientable surface with no inside or outside.

## The Math

We use the "Figure-8 Immersion" of the Klein Bottle.
The parameterization uses `u \in [0, 2\pi]` and `v \in [0, 2\pi]`.
Unlike a torus, the Klein bottle requires a "twist" when gluing the ends:
- The edge at `u=2\pi` connects to `u=0` but with `v` flipped (`2\pi - v`).
- This codebase implements this topological gluing in the index buffer generation, ensuring a seamless mesh even though the underlying coordinate system has a discontinuity in 3D space.

## Controls

- **WASD / Arrow Keys**: Move camera
- **QE**: Up/Down
- **Mouse Drag**: Rotate camera
- **Scroll**: Zoom

## Running

```bash
cargo run -p klein-files -- .
```

(Pass a directory path as an argument to scan it).

## Moonshot Features

- **Topologically Correct Gluing**: No visible seams despite the 4D nature of the object.
- **Neon Topology Shader**: The grid is colored based on its UV coordinates, highlighting the non-orientable flow.
