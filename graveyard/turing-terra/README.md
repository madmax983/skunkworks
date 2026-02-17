# Turing Terra ⚛️🌍

> "The mountains are not static; they are waves in a reaction-diffusion medium." - Genesis

**Turing Terra** is a moonshot experiment combining **Reaction-Diffusion Systems** (Gray-Scott) with **Terrain Generation**.
It simulates a living chemical soup where patterns (spots, stripes, chaos) determine the height and biome of the terrain in real-time.

## Concept
The world is a 200x200 grid where two chemicals, $U$ and $V$, react and diffuse.
- **$U$ (Prey)**: Represented as the "Substrate" or Lowlands.
- **$V$ (Predator)**: Represented as "Catalyst" or Mountains/Vegetation.

The concentration of $V$ drives the height of the terrain mesh and its color (Sand -> Green -> Coral).
Users can explore the phase space of the Gray-Scott model by adjusting the Feed ($F$) and Kill ($k$) rates, shifting the world from a barren desert to a labyrinthine coral reef.

## Controls
- **WASD + Q/E**: Fly through the world (Move X/Z/Y).
- **Arrow Keys**: Look around (Yaw/Pitch).
- **Click / Space**: Paint catalyst ($V$) at the crosshair location (Raycast to Y=0 plane).
- **J / K**: Decrease / Increase Feed rate ($F$).
- **U / I**: Decrease / Increase Kill rate ($k$).
- **1 / 2 / 3**: Load Presets (Solitons, Coral, Maze).
- **R**: Reset the simulation.

## Technical Details
- **Stack**: Rust, `macroquad` (Graphics), `rayon` (Parallel Simulation).
- **Simulation**: Parallelized Gray-Scott reaction-diffusion with toroidal boundary conditions (world wraps around).
- **Rendering**: Real-time mesh deformation. Vertices are updated every frame based on chemical concentration.

## Presets
1. **Solitons**: Isolated spots that grow and split.
2. **Coral**: Finger-print like patterns that grow organically.
3. **Maze**: Chaotic vermicular patterns constantly shifting.

## The Moonshot
This experiment proves that complex geological forms can emerge from simple local interaction rules, rather than explicit noise functions (like Perlin/Simplex). The terrain "grows" rather than being "generated".
