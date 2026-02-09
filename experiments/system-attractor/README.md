# System Attractor ⚛️⛈️

> "The weather of the machine."

Visualizing the chaotic dynamics of your computer's internal state using the Lorenz Attractor.

## The Concept

The **Lorenz Attractor** is a system of ordinary differential equations originally derived for atmospheric convection. It is a canonical example of deterministic chaos.

This experiment maps your system's "Vital Signs" to the Lorenz parameters:
- **CPU Usage** -> $\sigma$ (Sigma): Represents volatility/turbulence.
- **Memory Usage** -> $\rho$ (Rho): Represents the driving force.
- **Swap Usage** -> **Jitter**: Introduces entropy/noise into the particle positions.
- **Load Average** -> **Color Shift & Instability**: High load causes the visualization to glitch and shift hues.

## Controls

- **WASD**: Move Camera (Zoom/Pan)
- **Arrow Keys**: Rotate Camera
- **R**: Reset Simulation
- **ESC**: Quit

## Implementation Details

- **Particles**: 500,000 particles simulated in parallel using `rayon`.
- **Integration**: 4th Order Runge-Kutta (RK4).
- **Visualization**: `macroquad` with double-buffered accumulation rendering to create "trails" and "long exposure" effects.
- **Audio**: Real-time synthesis based on particle Z-position and velocity (if `audio` feature enabled).

## Running

```bash
cargo run --release
```

*Note: Release mode is recommended for 500k particles.*
