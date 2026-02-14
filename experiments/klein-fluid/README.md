# 🧬 Klein Fluid

**"Turbulence on a Twisted Manifold"**

A hybrid experiment combining Lattice Boltzmann Fluid Dynamics with the non-orientable topology of a Klein Bottle.

## Lineage

- **Parent A**: `typographic-turbulence` (LBM Fluid Logic)
- **Parent B**: `klein-flock` (Klein Bottle Topology)

## Concept

Simulating fluid dynamics on a surface that has no inside or outside.
The simulation runs on a grid where the left and right boundaries are identified with a **twist**.
This means that fluid flowing out the right side re-enters on the left side, but with its Y-coordinate inverted (`y' = H - y`) and its Y-velocity flipped (`vy' = -vy`).

This creates a non-orientable flow where vortices can collide with their own inverted reflections.

## Visuals

- **Red**: Positive Curl (Counter-Clockwise Vorticity)
- **Blue**: Negative Curl (Clockwise Vorticity)
- **White**: High Density

## Controls

- **Space**: Splash in the center.
- **W**: Inject "Wind" from the left boundary.
- **C**: Toggle Curl visualization vs Density.
- **R**: Reset simulation.
- **Q**: Quit.

## Technical Details

- Uses a D2Q9 Lattice Boltzmann Method (LBM).
- Implements a custom streaming step that handles the Klein Bottle twist.
- Renders to the terminal using `ratatui` and `Canvas` widget with Braille/Block resolution.
