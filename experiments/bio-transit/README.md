# Bio-Transit 🍄🚄

**"Slime mold pathfinding + Urban transit network design"**

## Concept
`bio-transit` simulates Physarum polycephalum (slime mold) agents acting as "Commuters" in a procedurally generated urban landscape.
Agents must travel between their Home and Work cities, depositing pheromones that attract other agents.
This creates emergent "Highways" and "Public Transit Lines" as agents reinforce efficient paths.

## Key Features
- **Physarum Logic:** Agents sense, turn, and move based on pheromone gradients (Jones 2010 model).
- **Commuter Behavior:** Agents have persistent Home/Work targets and switch states daily.
- **Target Bias:** Agents combine local gradient sensing with a global "GPS" pull towards their destination.
- **Visuals:**
    - Real-time trail diffusion and decay.
    - Color-coded pheromone intensity (Deep Blue -> Cyan -> White).
    - Glowing cities.

## Controls
- `Q`: Quit the simulation.

## Tech Stack
- **Rust**: Core logic.
- **Macroquad**: GPU visualization.
- **Rayon**: Parallel agent updates (50,000+ agents).
