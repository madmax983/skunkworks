# Crystal FS: The File System Crystallographer ⚛️💎

> "Crystals are frozen mathematics. Your data is a lattice waiting to be revealed."

**Crystal FS** visualizes your file system as a 4D hyper-crystal projected into 3D space.

## The Concept

Standard file explorers are flat trees. **Crystal FS** treats the file path as a sequence of vectors in a high-dimensional space.
- Each directory name seeds a unique 4D unit vector.
- A file's position is the vector sum of its path components: $\vec{P}_{file} = \sum \vec{v}_{dir}$.
- This creates a **Random Walk Lattice** in 4D space. Regular naming conventions create regular crystalline structures. Irregular names create amorphous solids.

## The Mathematics

We render points $(x, y, z, w)$ by projecting them onto a 3D hyperplane (the screen).
You can rotate the crystal not just in 3D (Spatial Rotation), but in 4D (Hyper-Rotation).
Rotating in 4D reveals the hidden structure of your data lattice—aligning Miller planes that were previously invisible.

## Controls

### 3D Navigation (Spatial)
- **W / S**: Move Forward / Backward
- **A / D**: Move Left / Right
- **Space / Shift**: Move Up / Down
- **Arrow Keys**: Look Around (Pitch / Yaw)

### 4D Navigation (Hyper-Rotation)
- **Q / E**: Rotate XW Plane (The "Time" axis vs "Width")
- **R / F**: Rotate YW Plane (The "Time" axis vs "Height")
- **T / G**: Rotate ZW Plane (The "Time" axis vs "Depth")

## Installation

```bash
cargo run -p crystal-fs
```

## Obsession
This project fulfills the mandate of **Genesis: The Crystallographer**: to render the impossible beauty of mathematical symmetry.
We have smashed together **Miller Indices** and **File System Visualization**.
Every folder is a facet. Every file is an atom.
