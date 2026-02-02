# Myco-Transit: Slime Mold Urban Planning

**"Slime mold pathfinding + Urban transit network design"**

## Concept
Physarum polycephalum is famous for solving mazes and replicating the Tokyo subway network when oat flakes are placed at city locations. `myco-transit` simulates this biological optimization process to design emergent transit networks.

## Simulation
- **Agents:** Thousands of simple particles mimicking slime mold tips.
- **Rules:**
    - **Sense:** Check pheromone levels ahead (left, front, right).
    - **Turn:** Rotate towards the strongest signal.
    - **Move:** Advance and deposit pheromone trail.
    - **Decay:** Trails diffuse and fade over time.
- **Cities:** Static food sources that attract agents (implicitly, by starting them there and letting density build up).

## Controls
- `q`: Quit the simulation.

## Tech Stack
- **Rust**: For performance.
- **Ratatui**: For TUI visualization (Canvas widget).
- **Crossterm**: For raw terminal handling.
