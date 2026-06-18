# Origami 🦢

A library for generating procedural origami meshes, specifically the **Miura-ori** fold.

The Miura-ori is a method of folding a flat surface such as a sheet of paper into a smaller area.
It consists of a tessellated pattern of parallelograms and is known for its property of
having a single degree of freedom—meaning the entire structure can be expanded or contracted
with a single motion.

## Core Concepts

*   **[`generate_miura_mesh`]**: The main generator function.
*   **[`MiuraParams`]**: Configuration for the geometric properties of the fold (unit cell dimensions, angle).
*   **Extension Factor**: A value from 0.0 (collapsed) to 1.0 (fully expanded) that drives the simulation.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
origami = "0.1.0"
```

## Example

```
use origami::{generate_miura_mesh, MiuraParams, Orientation};

// Define the fold parameters
let params = MiuraParams {
    a: 1.0,      // Side length A
    b: 1.0,      // Side length B
    gamma: 1.4,  // Fold angle (radians)
    orientation: Orientation::Horizontal,
};

// Generate the mesh at 50% expansion for a 10x10 grid
let mesh = generate_miura_mesh(params, (10, 10), 0.5);

assert_eq!(mesh.vertices.len(), 11 * 11); // (cols+1) * (rows+1)
```
