# Lensing Poetry ⚛️📜

A moonshot experiment combining N-Body gravitational simulation with text distortion effects (gravitational lensing).

## Concept
Mass bends space-time. Light follows the curvature. In this experiment, the "light" is the poetry of the cosmos written on the background. Massive bodies distort the text, revealing the gravitational wells.

## Controls
- **Left Click**: Add a Star (Massive Body)
- **Right Click**: Add Dark Matter (Negative Mass)
- **Space**: Reset to single central star

## Tech Stack
- `macroquad`: 2D rendering and game loop
- `glsl`: Custom fragment shader for ray deflection
- `symplectic integrator`: Stable orbital mechanics

## Physics
The simulation uses a Symplectic Euler integrator for energy conservation.
Deflection is calculated per pixel using a simplified Einstein deflection formula:
$$ \Delta \vec{x} \propto \sum \frac{M_i \vec{r_i}}{|\vec{r_i}|^2 + \epsilon} $$
