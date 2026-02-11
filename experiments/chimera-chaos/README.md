# Chimera Chaos 🧬🌪️

**Cross:** `chimera-lang` × `biotic-chaos`

## Concept
A hybrid experiment where `ChimeraVM` organisms inhabit a Coupled Map Lattice (CML) simulating chaotic dynamics.

The lattice runs a spatially coupled Logistic Map: `x(t+1) = r * x(t) * (1 - x(t))`.
The parameter `r` (Control Parameter) varies across the grid, creating regions of stability, bifurcation, and chaos.

Agents (ChimeraVMs) traverse this landscape:
- **Sensors:** They can read the local value (`x`) and the local chaos level (`r`).
- **Actuators:** They can move and *modify* the local `r` value, terraforming the chaos.
- **Metabolism:** They gain energy by "harvesting" the logistic map values.

## Controls
- **Left Click:** Spawn a new Agent at the mouse position.
- **Right Click:** Perturb the lattice (inject noise).
- **Space:** Pause simulation.
- **R:** Reset lattice and agents.

## Implementation Details
- **Lattice:** 400x300 grid updated in parallel using `rayon`.
- **Agents:** Each agent runs a `ChimeraVM` with a DNA program that drives its behavior.
- **Rendering:** `macroquad` renders the lattice state as a dynamic texture and agents as overlay particles.

## Emergence
Watch as agents cluster in high-energy regions or modify the terrain to suit their metabolic needs. The feedback loop between the biological agents and the chaotic substrate creates a complex adaptive system.
