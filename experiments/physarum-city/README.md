# Physarum City 🚇🦠

**"Slime mold pathfinding + Urban transit network design"**

## Concept
A simulation of *Physarum polycephalum* (slime mold) attempting to optimize a subway network.
Agents (slime) forage for food (stations), leaving behind pheromone trails that reinforce efficient paths.
Over time, the chaotic movement of thousands of agents crystallizes into a highly optimized transport network, mimicking how real slime molds can recreate the Tokyo subway system.

## Controls
- **Left Click**: Add a "Station" (Food Source). The slime will swarm to it.
- **R**: Reset simulation (Not implemented yet, restart app).

## How it Works
1. **Sensory Stage**: Each agent samples the pheromone grid at three forward positions (Left, Front, Right).
2. **Motor Stage**: The agent turns towards the strongest signal. If signals are equal, it moves straight or turns randomly.
3. **Deposition**: As agents move, they deposit pheromones onto the grid.
4. **Diffusion & Decay**: The pheromone grid diffuses (blurs) and decays over time, allowing old paths to fade and new optimal paths to emerge.

## Tech Stack
- **Rust** 🦀
- **Macroquad** 🎨 (Graphics)
- **Rayon** ⚡ (Parallelism for 50,000+ agents)

## Parameters
- `NUM_AGENTS`: 50,000
- `SENSOR_ANGLE`: 45 degrees
- `SENSOR_DIST`: 9 pixels
- `TURN_SPEED`: 0.2 radians/tick
