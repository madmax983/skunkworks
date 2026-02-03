# Voronoi Ants 🐜💎

A hybrid experiment combining `ant-colony` and `voronoi-relaxation`.

**Lineage:**
- **Parent A:** `experiments/ant-colony` (Ant agents, Pheromone logic, Scanning)
- **Parent B:** `experiments/voronoi-relaxation` (Voronoi tesselation rendering)
- **Role:** Splice Surgeon 🧬

## Concept
Ants forage for "Food" (TODO comments in your codebase). Each ant carries a "Territory Seed". The world is visualized as a dynamic Voronoi diagram where the regions are defined by the ants' positions.

As ants move to follow pheromone trails towards food or home, their territory shapes shift and morph fluidly.

- **Blue Regions:** Ants scouting for food.
- **Red Regions:** Ants carrying food back to the nest.
- **Green ☘:** Food Source (Code file with TODOs).
- **Yellow ⌂:** Nest (Center).

## Emergent Behavior
- The Voronoi cells act as a visual proxy for "Ant Density".
- When ants cluster around a food source, the cells become small and dense (High detail).
- When ants disperse, cells become large (Low detail).
- "Highways" appear as streams of rapidly shifting colored tiles.

## Controls
- `Q`: Quit

## Tech Stack
- `ratatui` (Direct Widget rendering)
- `crossterm`
- `walkdir` (Codebase scanning)
