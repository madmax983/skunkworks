# Ferrous Core

Core library for magnetic field and fluid density simulations in the Ferrous ecosystem.

`ferrous-core` provides shared data structures and algorithms used by various experiments
such as `ferrous-chimera`, `ferrous-fluid`, and `ferrous-graph`.

The central component is the [`Platter`], a 2D grid that can simulate:
- Magnetic field strength (clamped values).
- Fluid density (unbounded accumulation).
- Pheromone trails (with decay).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ferrous-core = { path = "../ferrous-core" }
```

## Quick Start

```rust
use ferrous_core::Platter;

// Create a 10x10 platter
let mut p = Platter::new(10, 10);

// Accumulate fluid density at (5, 5)
p.accumulate(5, 5, 1.5);

// Check the value
assert_eq!(p.get(5, 5), 1.5);

// Apply pheromone decay
p.decay(0.5);

assert_eq!(p.get(5, 5), 0.75);
```
