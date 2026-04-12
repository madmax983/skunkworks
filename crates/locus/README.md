# Locus 📍

A lightweight 2D/3D/4D geometry library for TUI applications, grid-based simulations, and hyper-dimensional experiments.

`locus` provides the fundamental primitives for moving, measuring, and mapping coordinates in discrete or continuous space.

## The Modules

- **`vec2`, `vec3`, `vec4`**: Robust mathematical vectors for physics and movement.
- **`flocking`**: Craig Reynolds' "Boids" algorithm for simulating complex group behavior.
- **`topology`**: The rulebook for how your world connects (Plane, Torus, Klein Bottle, etc.).

## The Hero's Journey: Navigating the Unknown

First, add `locus` to your `Cargo.toml`:

```toml
[dependencies]
locus = { path = "crates/locus" }
```

If you're building a simulation where agents move through space, `locus` handles the heavy lifting. This example shows an agent moving in a Torus world (where walking off the edge wraps you around to the other side).

```rust
use locus::{Vec2, Topology};

fn main() {
    let width = 20;
    let height = 10;

    // 🗺️ The Map Room
    // A Torus topology means the world wraps around like Pac-Man
    let topo = Topology::Torus;

    // 🕊️ The Agent
    // Start at position (x=19.0, y=5.0) - right at the eastern edge
    let mut position = Vec2::new(19.0, 5.0);

    // Moving east (right)
    let velocity = Vec2::new(1.0, 0.0);

    // Time steps forward...
    position += velocity;

    // We reached x=20.0, but our map is only 20 units wide (0 to 19)!
    // Use Topology to find our true grid cell coordinates.
    // Note: Topology expects (row, col) i.e. (y, x) integers.
    let y_idx = position.y.round() as i64;
    let x_idx = position.x.round() as i64;

    if let Some((ny, nx)) = topo.normalize(y_idx, x_idx, width, height) {
        // We safely wrapped to the left side!
        assert_eq!(nx, 0);
        assert_eq!(ny, 5);
        println!("Wrapped around to: x={}, y={}", nx, ny);
    }
}
```

## Features

- **`serde`**: (Optional) Enables `Serialize` and `Deserialize` on core types.
- **`macroquad`**: (Optional) Implements `From` and `Into` for interoperability with `macroquad::prelude::Vec3`.
