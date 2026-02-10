# Locus 📍

**Locus** is a lightweight, zero-dependency geometry library designed for TUI applications, cellular automata, and grid-based simulations.

It provides a robust `Vec2` implementation and a `Topology` enum for handling complex grid wrappings (Torus, Klein Bottle, Möbius Strip, etc.).

## Features

*   **`Vec2`**: A simple 2D vector struct with `f64` components.
*   **`Topology`**: Handles coordinate normalization and wrapping for various 2D manifolds.
*   **`serde` support**: Optional feature for serialization.

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
locus = "0.1"
```

### Vector Arithmetic

```rust
use locus::Vec2;

fn main() {
    let p1 = Vec2::new(10.0, 5.0);
    let velocity = Vec2::new(1.0, -0.5);

    // Move point
    let p2 = p1 + velocity;

    assert_eq!(p2, Vec2::new(11.0, 4.5));

    // Distance
    let dist = p1.distance(p2);
    assert!((dist - 1.118).abs() < 0.001);
}
```

### Topologies

Handle wrapping logic for exotic grids easily.

```rust
use locus::Topology;

fn main() {
    let grid_size = 10;

    // 1. Torus (Pac-Man world)
    let torus = Topology::Torus;
    // Moving off the right edge (x=10) wraps to left (x=0)
    assert_eq!(torus.normalize(5, 10, grid_size), Some((5, 0)));

    // 2. Möbius Strip
    let mobius = Topology::Mobius;
    // Moving off the right edge wraps to left BUT flips Y coordinate
    // (y=2 becomes y = 10 - 1 - 2 = 7)
    assert_eq!(mobius.normalize(2, 10, grid_size), Some((7, 0)));
}
```

## Supported Topologies

| Topology | Description | Wrap X | Wrap Y |
|----------|-------------|--------|--------|
| `Plane` | Bounded grid. | No | No |
| `Torus` | Standard wrap. | Yes | Yes |
| `CylinderH` | Horizontal cylinder. | Yes | No |
| `CylinderV` | Vertical cylinder. | No | Yes |
| `Klein` | Klein Bottle. | Yes | Yes (Twist) |
| `Mobius` | Möbius Strip. | Yes (Twist) | No |
| `Hyperbolic`| Poincaré Disk placeholder. | No | No |

## License

MIT
