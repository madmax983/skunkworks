# Poincaré Disk Model

A library for exploring the **Poincaré Disk Model** of hyperbolic geometry, where an entire infinite universe is compressed into the interior of a unit circle ($|z| < 1$).

This crate provides the mathematical primitives for working with hyperbolic points, isometries (Möbius transformations), and regular tilings.

## Quick Start

```rust
use poincare_disk::{Point, mobius_add, hyperbolic_dist};

fn main() {
    // 1. Create a point at the origin (center of the disk)
    let center = Point::new(0.0, 0.0);

    // 2. Create another point at (0.5, 0.0)
    let target = Point::new(0.5, 0.0);

    // 3. "Add" them together using Möbius addition.
    // In Euclidean space, 0 + 0.5 = 0.5.
    // In Hyperbolic space, adding a point 'a' to 'z' is a translation that moves 'z' by 'a'.
    let result = mobius_add(center, target);

    // 4. Measure the distance.
    // Euclidean distance is 0.5.
    // Hyperbolic distance is 2 * atanh(0.5) ≈ 1.0986.
    let dist = hyperbolic_dist(center, result);

    println!("Hyperbolic Distance: {}", dist);
    assert!((dist - 1.0986).abs() < 0.001);
}
```

## Core Concepts

### Points
Points are represented as complex numbers `Complex<f64>` strictly inside the unit disk ($|z| < 1$). The boundary ($|z| = 1$) represents infinity and is unreachable.

### Möbius Transformations
Motions in hyperbolic space (translations and rotations) are represented by **Möbius transformations** of the form:
$$ f(z) = \frac{az + b}{cz + d} $$
These transformations map the unit disk to itself and preserve hyperbolic distances (they are isometries).

- **Translation:** Moving a point $z$ by $a$ is done via `mobius_add(z, a)`: $\frac{z+a}{1+\bar{a}z}$.
- **Rotation:** Standard complex multiplication $e^{i\theta}z$.

### Geodesics
Straight lines in hyperbolic geometry (geodesics) appear as circular arcs orthogonal to the boundary of the disk.

## Tiling

The library includes constants for generating regular hyperbolic tilings, such as the `{4, 5}` tiling (squares where 5 meet at each vertex).

```rust
use poincare_disk::{TilingConsts, neighbor_transform_a};

let consts = TilingConsts::new_4_5();

// Get the translation required to move to the neighbor on the right (index 0)
let right_step = neighbor_transform_a(0, &consts);

// To render the tiling, you would recursively apply these translations to your view.
```

## License

MIT
