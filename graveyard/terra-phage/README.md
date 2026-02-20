# Terra Phage ⚛️
> "The map is not the territory... the territory is eating the map."

**Terra Phage** is a **Reaction-Diffusion Terrain Generator**. It simulates a Gray-Scott chemical system on a grid and maps the chemical concentrations to terrain height and biome color in real-time.

![Concept](https://upload.wikimedia.org/wikipedia/commons/e/e6/Reaction-diffusion_Gray-Scott.png)
*(Standard Gray-Scott pattern, visualized as 2D)*

## Concept
- **U (Substrate)**: Maps to **Height**. High concentration = Lowlands/Water, Low concentration = Mountains (or vice-versa depending on the regime).
- **V (Catalyst)**: Maps to **Biome/Toxicity**. High concentration = Magma/Alien growth.
- **Interaction**: The user can disrupt the system by "raining" catalyst or "bombing" areas, forcing the terrain to regrow and adapt.

## Controls
- **Right Click + Drag**: Orbit Camera.
- **Scroll**: Zoom In/Out.
- **Space**: **Rain** (Drop catalyst in random spots).
- **Enter**: **Bomb** (Drop massive catalyst load).
- **Arrow Up/Down**: Adjust **Feed Rate** (f).
- **Arrow Left/Right**: Adjust **Kill Rate** (k).
- **R**: **Reset** simulation.

## Parameters
The simulation starts in the "Coral" regime:
- Feed: `0.0545`
- Kill: `0.0620`

Try exploring:
- **Solitons**: Feed `0.03`, Kill `0.062`
- **Maze**: Feed `0.029`, Kill `0.057`

## Architecture
- **Engine**: `macroquad` (0.4)
- **Simulation**: `gray-scott` crate (Parallelized via `rayon`)
- **Rendering**: Dynamic `Mesh` generation with vertex coloring.
