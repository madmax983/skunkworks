# ⏳ Sandpile Scheduler: Self-Organized Criticality Load Balancer

**"Avalanches are just aggressive load balancing."**

A Moonshot experiment combining **Abelian Sandpile Model** (physics) with **Distributed Task Scheduling** (systems).

## ⚛️ The Concept

In a distributed system, tasks (grains of sand) arrive at nodes. Each node has a limited capacity (slope). When capacity is exceeded, the node "topples", distributing tasks to its neighbors.

This experiment explores:
- **Emergent Load Balancing**: How a simple local rule (topple if > 3) creates global stability.
- **DDoS Visualization**: What happens when tasks arrive faster than the system can relax? The "Avalanche" represents a cascading failure or a load storm.
- **Self-Organized Criticality**: The system naturally evolves to a critical state where avalanches of all sizes can occur.

## 🕹️ Controls

- **[Space]**: Pause/Resume simulation.
- **[D]**: Toggle **DDoS Mode** (Continuous random packet injection).
- **[R]**: Reset the grid.
- **[Click]**: Manually inject a "Packet Burst" (50 tasks) at the cursor.
- **[Up/Down]**: Adjust "Processing Rate" (Task completion speed).

## 🧪 Simulation Details

- **Grid**: 512x512 nodes.
- **Physics**: Parallelized Abelian Sandpile Model (using `rayon`).
- **Processing**: Each node randomly processes (consumes) tasks based on `Process Rate`.
- **Visualization**:
  - ⬛ **Black**: Idle (0 tasks)
  - 🟦 **Blue**: Low Load (1-3 tasks)
  - ⬜ **White**: Overload (Toppling imminent)
  - 🟥 **Red**: Critical Mass (Massive backlog)

## 🚀 Running

```bash
cargo run -p sandpile-scheduler
```

## 🐜 Genesis Notes

This experiment smashes together **Locust Swarms** (DDoS attacks) and **Termite Mounds** (emergent structures). The "sand" behaves like a liquid swarm, routing itself around obstacles (if implemented) or flooding the path of least resistance.
