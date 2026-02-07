# Phase Engine

**"Crystallization is just a loss of entropy."**

A real-time simulation of phase transitions (Solid <-> Liquid) using Langevin Dynamics.
Particles are simulated on the CPU (using `rayon` for parallelism) and rendered on the GPU (using `wgpu` instancing).

## Concept

Each particle is tethered to a lattice position by a spring force (simulating solid state) but perturbed by random thermal noise (simulating temperature).

- **Low Temperature**: Spring forces dominate. Particles vibrate around lattice points (Crystal/Solid).
- **High Temperature**: Thermal noise dominates. Particles break free and move randomly (Liquid/Gas).

Equation of Motion (Langevin):
$$ F = -k(x - x_{lattice}) - \gamma v + \eta(t) $$
Where $\eta(t)$ is white noise proportional to Temperature.

## Tech Stack
- **wgpu**: Graphics API.
- **rayon**: Parallel CPU simulation.
- **cgmath**: Vector math.
- **winit**: Windowing.

## Controls
- **Arrow Up**: Increase Temperature (Melt).
- **Arrow Down**: Decrease Temperature (Freeze).
- **R**: Reset Simulation.
- **W/A/S/D**: Move Camera.
- **Space/Shift**: Move Up/Down.
