# 🍄🥩 Myco-Tissue: Elastic Pheromone Network

A hybrid experiment combining `myco-transit` (Slime Mold Pheromone Pathfinding) and `ferrous-tissue` (Soft Body Mechanics / PBD).

## 🧬 Concept

Physarum polycephalum foragers seek food by depositing and sensing pheromones. However, in `myco-tissue`, the agents are not a liquid swarm. They are physically tethered together by elastic springs (Position Based Dynamics constraints), forming a "tissue."

The desire to forage (chemotaxis) is constantly at odds with the organism's structural integrity (spring tension).

## 🔬 Lineage

- **Parent A (`experiments/myco-transit`)**: The `Pheromone Grid` memory, gradient calculation, and trail decay.
- **Parent B (`experiments/ferrous-tissue`)**: Soft-body simulation using internal spring constraints to prevent the organism from ripping apart.

## 🎮 Interaction

- Watch as the tissue crawls toward high-pheromone regions (simulating food) like a coordinated inchworm.
- **Space**: Reset the simulation.
