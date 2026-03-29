# Thermo-Defense ⚛️🐜🦗

> "The Colony is under siege. The Servers must remain cool."

**Thermo-Defense** is a Moonshot experiment combining **Termite Mound Ventilation** and **Locust Swarm Attacks** into a real-time defense simulation.

## The Concept

A Data Center is the battleground.
- **Servers**: Generate Heat. If they overheat, the system fails (conceptually).
- **Termites (Blue/Yellow)**: Defenders. They build cooling structures to dissipate heat but must also build walls to block attackers.
- **Locusts (Green)**: Attackers (DDoS packets). They seek heat and try to destroy walls and overheat servers.

This is a study in **Emergent Defense**: Can a decentralized swarm of simple agents build effective fortifications against an adaptive enemy?

## How to Run

```bash
cargo run --release -p thermo-defense
```

## Controls

- **Left Click**: Spawn a Server (Heat Source).
- **Right Click**: Spawn a Wall (Obstacle).
- **T**: Spawn 100 Termites.
- **L**: Spawn 100 Locusts.
- **R**: Reset Simulation.

## The Science

- **Heat Diffusion**: Modeled as a 2D cellular automaton.
- **Stigmergy**: Termites communicate via Pheromones (Defense) and Heat gradients.
- **Swarm Intelligence**:
    - Termites use local rules to decide whether to build or fight.
    - Locusts use gradient descent to find targets.
- **Parallelism**: The simulation uses `rayon` to update thousands of agents in parallel, decoupling decision-making from world-state mutation.

## Legend

- **Red**: Server (Heat Source) / Heat Map.
- **White**: Wall.
- **Blue**: Termite (Empty).
- **Yellow**: Termite (Carrying Wall Block).
- **Green**: Locust.
- **Cyan/Purple Haze**: Pheromones.
