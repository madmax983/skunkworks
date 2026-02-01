# Urban Myco-Web: Hyphal Commute

**Status:** Experimental ⚛️🍄
**Stack:** `ratatui`, `rand`

## Concept
"Hyphal Commute" simulates a slime mold (Physarum polycephalum) solving a transportation network design problem.
We treat "Population Centers" (Stations) as food sources. The simulation releases "Agents" (Hyphae) that seek nutrients.
Over time, the agents form efficient transport networks connecting the stations, mirroring how slime molds map efficient routes (like the famous Tokyo Railway experiment).

## How it works
- **Agents:** Simple particles that move, deposit pheromones, and sense gradients.
- **Grid:** A pheromone trail map that diffuses and decays.
- **Stations:** Emitters that constantly output high pheromone levels.

## Controls
- `q`: Quit
- `r`: Reset Simulation
- `g`: Generate new random city layout
