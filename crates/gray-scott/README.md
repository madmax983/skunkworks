# Gray-Scott Reaction-Diffusion

A high-performance simulation of the Gray-Scott reaction-diffusion system.

This crate provides the `GrayScott` struct, which simulates two virtual
chemicals (U and V) diffusing and reacting on a 2D grid. Depending on the
feed and kill rates, this system can generate complex, life-like patterns
such as spots, stripes, and dividing cells.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
gray-scott = { path = "../gray-scott" }
```

## Features
- **`parallel`**: (Optional) Enables multi-threaded updates using `rayon` for significant performance gains on large grids.

## Usage

```rust
use gray_scott::GrayScott;

// Create a small dish
let mut dish = GrayScott::new(100, 100);

// Drop a "spore" of chemical V in the center
dish.add_chemical(50, 50, 1.0);

// Simulate "Cell Division" parameters over time.
let (feed, kill) = (0.0367, 0.0649);

for _ in 0..100 {
    dish.update(feed, kill, 1.0);
}
```
