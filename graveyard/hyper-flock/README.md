# Hyper-Flock

**Experiment**: `hyper-flock`
**Parents**: `experiments/tesseract-ops` + `experiments/luminous-flock`

## Concept

Boids flocking in a 4D Hypercube where the dimensions of the universe are driven by system metrics.

- **CPU Usage** drives the **X-axis** scale.
- **Memory Usage** drives the **Y-axis** scale.
- **Swap Usage** drives the **Z-axis** scale.
- **System Load** drives the **W-axis** scale (breathing/rotation speed).

## Lineage

- **From `tesseract-ops`**: The 4D `Vec4` math, the `SystemMonitor` logic, and the Tesseract projection visualization.
- **From `luminous-flock`**: The Boid steering behaviors (Separation, Alignment, Cohesion), adapted for 4D Euclidean space.

## Emergent Behavior

The flock lives in a universe that expands and contracts based on the computer's effort. When the system is under load (high CPU/RAM), the "room" gets bigger, and the boids have more space to separate. When the system is idle, the universe shrinks, forcing the boids into a tighter, more chaotic cluster.

The 4th dimension (W) acts as a "hidden" depth. Boids might appear to pass through each other in 3D, but are actually separated in W. The color of the boids fades based on their W-coordinate, giving a visual cue of their hyper-depth.
