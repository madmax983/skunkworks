# Gray-Scott Reaction-Diffusion

A high-performance simulation of the Gray-Scott reaction-diffusion system.

This crate provides the `GrayScott` struct, which simulates two virtual
chemicals (U and V) diffusing and reacting on a 2D grid. Depending on the
feed and kill rates, this system can generate complex, life-like patterns
such as spots, stripes, and dividing cells.

## Core Mechanism

The simulation models two equations:

- `dU/dt = D_u * Laplace(U) - U * V^2 + f * (1 - U)`
- `dV/dt = D_v * Laplace(V) + U * V^2 - (f + k) * V`

Where:
- `D_u, D_v`: Diffusion rates for chemicals U and V.
- `Laplace()`: The Laplacian operator (calculated via a 3x3 convolution kernel).
- `U * V^2`: The reaction where two V molecules and one U molecule turn into three V molecules.
- `f`: Feed rate (replenishes U).
- `k`: Kill rate (removes V).

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
fn main() {
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
}
```
