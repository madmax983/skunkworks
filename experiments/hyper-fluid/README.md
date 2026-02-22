# Hyper Fluid

**Parents:** `ferrous-fluid` × `tesseract-ops` (with `hyper-mold` lineage)

A 4D Fluid Simulation where the "medium" is distorted by real-time system metrics.

## Concept

Imagine a fluid trapped in a 4-Dimensional Hypercube (Tesseract).
The physics of this fluid are driven by the computer's own metabolism:

*   **CPU Usage** drives **Agitation (Temperature)**. High CPU load makes particles vibrate violently.
*   **Memory Usage** drives **Viscosity**. High RAM usage makes the fluid thick and sluggish (high damping).
*   **Swap Usage** drives **Gravity**. High Swap usage creates a heavy gravitational pull towards the "bottom" of the 4th dimension (or Y-axis).
*   **Load Average** drives the **Rotation** of the Tesseract itself.

## Implementation

*   **Particle System**: 4D particles with position, velocity, and acceleration.
*   **Grid4D**: A 4D grid stores particle density to calculate "pressure" gradients, pushing particles away from high-density clusters (mimicking fluid incompressibility).
*   **Rendering**: 4D coordinates are projected to 3D space for visualization using `macroquad`. The 4th dimension (W) is visualized as color.

## Controls

*   **Arrow Keys / WASD**: Rotate and Zoom Camera.
*   The Tesseract rotates automatically based on system load.

## Lineage

*   Inherits **Magnetic/Fluid Physics** logic from `ferrous-fluid` (but adapted to 4D particles).
*   Inherits **4D Visualization & System Monitoring** from `tesseract-ops`.
*   Uses **4D Grid/Math** infrastructure from `hyper-mold`.
