# Hyper System

This crate provides the core mathematical and system monitoring utilities for the "Hyper" series of experiments. It encapsulates shared logic to ensure consistency across different visualizations and simulations.

## Modules

- **`math`**: Provides `Vec4`, a 4D vector struct with operations for:
  - Vector arithmetic (addition, subtraction, scaling).
  - 4D geometric transformations (rotation in XW, YW, ZW planes).
  - Perspective projection from 4D to 3D space.

- **`monitor`**: Provides `SystemMonitor`, a utility for tracking real-time system metrics:
  - CPU usage.
  - Memory usage.
  - Swap usage.
  - Load average.
  - Smooth interpolation of metrics for fluid visualization updates.

## Usage

Add this crate as a dependency in your `Cargo.toml`:

```toml
[dependencies]
hyper-system = { path = "../hyper-system" }
```

Then use it in your code:

```rust
use hyper_system::math::Vec4;
use hyper_system::monitor::SystemMonitor;

let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
let mut monitor = SystemMonitor::new();
monitor.update();
```
