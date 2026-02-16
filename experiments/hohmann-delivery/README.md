# Hohmann Delivery ⚛️🚀📦

> "Why deliver in a straight line when you can surf the gravity well?" — Genesis (The Astronomer)

**Hohmann Delivery** is a Moonshot experiment combining orbital mechanics with resource delivery optimization.
It simulates interplanetary logistics where the cost of transport ($\Delta v$) and time-of-flight fluctuate based on planetary alignments.

## Concept

In space, the shortest distance between two points is rarely a straight line. It's an ellipse.
And the most efficient way to travel between orbits is the **Hohmann Transfer**.
However, this transfer is only possible during specific **Launch Windows**.

This simulation challenges you to:
1.  **Monitor Phase Angles**: Wait for the target planet to be in the correct position relative to the source.
2.  **Calculate $\Delta v$**: Determine the precise velocity change needed to enter the transfer orbit.
3.  **Optimize Logistics**: Balance fuel costs (waiting for ideal window) vs delivery speed (powered transfers).

## Tech Stack

*   **Engine**: `macroquad` (Rust) for 2D visualization.
*   **Physics**: Symplectic Integrator (Velocity Verlet) for stable N-Body simulation.
*   **Guidance**: Analytical solutions for Keplerian transfer orbits.

## Controls

*   **Space**: Pause / Resume Simulation.
*   **Left / Right Arrow**: Adjust Simulation Speed (Time Scale).
*   **Click (Left)**: Select a Planet to view transfer options.
*   **Enter**: Launch a ship if a window is open.

## Running

```bash
cargo run -p hohmann-delivery
```
