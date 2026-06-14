# Platter

A 2D grid structure optimized for simulating fields like magnetism, density, or pheromones.

The `platter` crate provides the [`Platter`] struct, which is designed to efficiently
store and update field values across a 2D grid. It includes methods for accumulating values,
hard-capping saturation, and applying time-based decay.

## Quick Start

```rust
use platter::Platter;

fn main() {
    // Create a 10x10 platter
    let mut p = Platter::new(10, 10);

    // Accumulate some value at (5, 5)
    p.accumulate(5, 5, 0.5);

    // Check the value
    assert_eq!(p.get(5, 5), 0.5);

    // Apply decay
    p.decay(0.9);

    // Check the decayed value
    assert_eq!(p.get(5, 5), 0.45);
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
platter = { path = "../platter" }
```
