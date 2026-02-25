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

## Philosophy 🧠

The "Hyper" series experiments are about visualizing higher-dimensional spaces and using the computer's own biological signs (CPU, RAM) to drive the simulation.

1.  **4D as First-Class Citizen**: We don't just hack 3D vectors. We use `Vec4` to perform true 4D rotations before projecting down to 3D.
2.  **System as Input**: The `SystemMonitor` allows the artwork to react to the machine running it. A stressed CPU might cause a chaotic, high-energy visualization, while an idle machine produces calm, slow-moving patterns.

## Usage

Add this crate as a dependency in your `Cargo.toml`:

```toml
[dependencies]
hyper-system = { path = "../hyper-system" }
```

### Example: 4D Rotation & Projection

```rust
use hyper_system::math::Vec4;
use std::f32::consts::PI;

fn main() {
    // 1. Create a point in 4D space
    let point = Vec4::new(1.0, 0.0, 0.0, 0.0);

    // 2. Rotate it in the XW plane (swapping X and W)
    // This is a rotation that doesn't exist in 3D space!
    let rotated = point.rotate_xw(PI / 2.0);

    // 3. Project it down to 3D for rendering
    // We place a "camera" on the W-axis at w=5.0
    let projected_3d = rotated.project_to_3d(5.0);

    println!("Projected: {:?}", projected_3d);
}
```

### Example: System Monitoring

```rust
use hyper_system::monitor::SystemMonitor;

fn main() {
    let mut monitor = SystemMonitor::new();

    // In your game loop:
    // monitor.update(); // If using macroquad
    // OR
    // monitor.update_with_time(dt, current_time);

    println!("CPU Stress: {:.2}%", monitor.cpu_usage * 100.0);
}
```
