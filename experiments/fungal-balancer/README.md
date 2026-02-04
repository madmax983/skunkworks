# Fungal Balancer 🍄⚖️

**"Mycelium nutrient sharing + Load balancer simulation"**

A biological simulation of distributed load balancing.

## The Concept

In this simulation, servers are fungal colonies and requests are nutrients. The network grows organically to balance load across the system.

- **Nodes (Servers):** Emulate fungal colonies. They have a `Load` and `Capacity`.
  - **Stressed Nodes** (Load > Capacity) release spores (Agents).
  - **Idle Nodes** (Load < Capacity) release pheromones (Capacity Signal).
- **Agents (Hyphae/Packets):**
  - Spawn from stressed nodes carrying load.
  - Navigate the grid sensing Capacity Pheromones and existing Trails.
  - Deposit load when they find a node with spare capacity.
  - Leave a trail (mycelium) behind them, reinforcing the path for future agents.

## Implementation

- **Language:** Rust
- **Visuals:** `ratatui` (Terminal User Interface)
- **Algorithm:** Physarum-like agent-based model (Slime Mold) + Gradient Diffusion.

## Controls

- `q`: Quit
- `r`: Reset simulation (randomize nodes)
- `s`: Spike Load (Add massive load to a random node)

## Running

```bash
cargo run -p fungal-balancer
```

## Observation

Watch as the system self-organizes. When a node is overloaded (Red), it sends out packets (Yellow dots) that seek Green nodes. Over time, stable "highways" (White trails) emerge between sources of load and sinks of capacity.
