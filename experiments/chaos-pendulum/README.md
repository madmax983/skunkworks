# Dependency Chaos Pendulum ⚛️🕰️

> "If you wish to make an apple pie from scratch, you must first invent the universe." — Carl Sagan
> "If you wish to build a Rust crate, you must first download half of crates.io." — Genesis (The Cynic)

**Chaos Pendulum** visualizes your Rust dependency graph as a massive, multi-jointed physical pendulum system.

## The Concept

This experiment maps your `cargo` dependency tree to a physical system:
- **Nodes**: Crates (libraries).
- **Links**: Dependency relationships.
- **Mass**: Proportional to the number of dependencies the crate has. Heavier crates pull harder.
- **Anchor**: The root crate (the one you are running) is fixed in space.

Because this is a multi-pendulum system with hundreds of joints, it exhibits **Deterministic Chaos**.
To visualize this, we simulate 100 "Ghost" universes in parallel. Each ghost starts with imperceptibly tiny variations in position.
Over time, due to the Butterfly Effect (sensitive dependence on initial conditions), the ghosts diverge from the "Real" universe, creating a cloud of probability.

## 🎮 Controls

| Key | Action |
| :--- | :--- |
| **`W` / `A` / `S` / `D`** | Pan Camera |
| **`+` / `-`** | Zoom In / Out |
| **`↑` / `↓`** | Adjust **Gravity** (Y-axis) |
| **`←` / `→`** | Adjust **Friction** (Damping) |
| **`K`** | **KICK** the system (Add Chaos) |
| **`R`** | **Reset** Simulation |
| **`G`** | Toggle **Ghosts** visibility |
| **`Space` + Drag** | Drag all ghosts together with the real node |
| **Mouse Drag** | Drag a specific node (Real universe only) |

## 🌈 Visualization

- **🔴 Red Nodes**: Fixed Anchors (Root crate).
- **🔵 Blue Nodes**: Intermediate dependencies (Heavy).
- **🟢 Green Nodes**: Leaf dependencies (Light).
- **Cyan Lines**: Ghost trajectories (Probability cloud).
- **Metrics**: "Divergence" measures the average distance between the Real Universe and the Ghost Universes. Higher = More Chaos.

## 📦 Usage

Run from the root of a workspace to visualize that workspace's dependencies.

```bash
cargo run -p chaos-pendulum
```
