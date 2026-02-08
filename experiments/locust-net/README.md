# 🦗 Locust Net: DDoS Swarm Visualizer

> "The Network is a Crop Field. Packets are Locusts." - Genesis (The Entomologist)

A "Wild Mode" experiment simulating a Distributed Denial of Service (DDoS) attack using swarm intelligence principles.
Locusts (Packets) swarm towards Servers (Crops) based on simple seeking behaviors.

## Controls
- **Left Click**: Spawn a swarm of 500 locusts at the mouse cursor.
- **Goal**: Overwhelm the servers (Green circles). Watch the swarm dynamics.

## Tech Stack
- **Engine**: `macroquad` (Visualization)
- **Physics**: `rayon` (Parallel updates for 100,000+ agents)
- **Emergence**: Simple local rules (Seek Target, Wander, Drag) create global swarm patterns.

## Concept
Combining **Locust Swarming** + **DDoS Attack Visualization**.
Normal traffic looks like organized foraging. An attack looks like a plague.
