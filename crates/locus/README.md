# Locus 📍

A lightweight 2D geometry library for TUI applications and grid-based simulations.

`locus` provides the fundamental primitives for moving, measuring, and mapping coordinates in discrete or continuous space.

## Features

- **`Vec2`**: A robust 2D vector struct for physics and movement. Supports standard arithmetic, normalization, reflection, and rotation.
- **`Topology`**: A system for defining how your world wraps (Plane, Torus, Klein Bottle, Möbius Strip, etc.).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
locus = { path = "crates/locus" }
# or if using workspace:
locus = { workspace = true }
```

## Usage

### The Hero's Journey: Moving on a Torus

Here is a minimal example of moving a particle in a wrapping world (like in *Pac-Man* or *Asteroids*).

```rust
use locus::{Vec2, Topology};

fn main() {
    let width = 20;
    let height = 10;
    let topo = Topology::Torus;

    // Start at position (x=19.0, y=5.0) - at the right edge
    let mut position = Vec2::new(19.0, 5.0);
    let velocity = Vec2::new(1.0, 0.0); // Moving right

    // Move
    position += velocity;

    // Check raw position (now 20.5, 5.0) - out of bounds!
    println!("Raw Position: {:?}", position);

    // Normalize using Topology
    // Note: Topology expects (row, col) i.e. (y, x) integers for discrete grids.
    // For continuous coordinates, you might map them to indices.

    let y_idx = position.y.round() as i64;
    let x_idx = position.x.round() as i64;

    if let Some((ny, nx)) = topo.normalize(y_idx, x_idx, width, height) {
        println!("Normalized Grid Index: ({}, {})", ny, nx);
        // Should wrap to (5, 0)
        assert_eq!(nx, 0);
        assert_eq!(ny, 5);
    }
}
```

### Vector Math

`Vec2` supports standard vector operations.

```rust
use locus::Vec2;

let v1 = Vec2::new(3.0, 4.0);
let v2 = Vec2::new(1.0, 2.0);

let sum = v1 + v2;
let mag = v1.magnitude(); // 5.0
let unit = v1.normalize(); // (0.6, 0.8)
```
