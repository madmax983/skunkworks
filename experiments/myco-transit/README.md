# Myco-Transit: Slime Mold Urban Planning

**"Slime mold pathfinding + Urban transit network design"**

## Concept
Physarum polycephalum is famous for solving mazes and replicating the Tokyo subway network when oat flakes are placed at city locations. `myco-transit` simulates this biological optimization process to design emergent transit networks.

This simulation models "Commuter Slime Molds":
1.  **Agents** have a Home City and a Work City (Target).
2.  **Movement** is driven by two forces:
    -   **Chemotaxis (Trail Following):** Agents follow existing trails, reinforcing popular routes (Highways).
    -   **Gradient Descent (Goal Seeking):** Agents have a vague sense of direction towards their target city.
3.  **Result:** Agents naturally form efficient networks that balance direct paths (Euclidean) with shared infrastructure (Trails).

## Simulation
- **Agents:** ~10,000 agents running in parallel (using `rayon`).
- **Rules:**
    - **Sense:** Check pheromone levels + Gradient vector.
    - **Turn:** Rotate towards the strongest signal.
    - **Move:** Advance and deposit pheromone trail.
    - **Decay:** Trails diffuse and fade over time.
    - **Commute:** Upon reaching the destination, the agent rests and then reverses direction (Work -> Home).

## Controls
- `q`: Quit the simulation.

## Tech Stack
- **Rust**: For performance.
- **Rayon**: Parallel agent updates.
- **Ratatui**: For TUI visualization (Canvas widget).
- **Crossterm**: For raw terminal handling.
