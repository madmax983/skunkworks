# Lyapunov Sensor ⚛️🦋

> "The sensory organ for chaos."

A real-time visualization of the **Largest Lyapunov Exponent (LLE)** of a chaotic system driven by your computer's internal state.

## The Concept

The **Lyapunov Exponent** measures the rate of separation of infinitesimally close trajectories.
- $\lambda > 0$: Chaos (Trajectories diverge exponentially).
- $\lambda < 0$: Stability (Trajectories converge).

This experiment simulates a **Lorenz Attractor** whose parameters are modulated by your system's "vital signs":
- **CPU Usage** -> $\rho$ (Rho): The driving force. Low load = Stable Spiral. High load = Chaotic Butterfly.
- **Memory Usage** -> $\beta$ (Beta): The geometric shape.

Simultaneously, a "Shadow Particle" tracks the main particle to calculate the *local* divergence in real-time. The trail color represents this instantaneous stability.

## Visuals
- **Blue Trail**: Stable / Contracting region.
- **Red Trail**: Unstable / Expanding region (Chaos).
- **HUD**: Shows the global LLE average and current System Load.

## Controls
- **Arrow Keys**: Orbit Camera.

## Running
```bash
cargo run --release
```
