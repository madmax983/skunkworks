# ⏳ Chimera Sandpile: The Avalanche Surfers 🧬

> "We do not move the sand. The sand moves us."

A Hybrid Experiment combining **ChimeraVM** (Genetic Programming) with **Abelian Sandpile Model** (Self-Organized Criticality).

## 🧬 The Concept

In this simulation, ChimeraVM agents inhabit a 2D grid that behaves like a sandpile.
- **The World**: A grid where each cell has a "Load" (grains of sand). If Load >= 4, the cell topples, distributing sand to neighbors.
- **The Agents**: Living programs that can "See" the local sand height and "Terraform" it (Add/Remove sand).
- **The Physics**:
  - **Avalanche Surfing**: Agents trapped on a toppling cell are swept away to a random neighbor.
  - **Terraforming**: Agents evolve strategies to build structures or trigger avalanches to move.

## 🕹️ Controls

- **[R]**: Reset the simulation.
- **[Esc]**: Quit.

## 🧪 Simulation Details

- **Grid**: 128x128 (default).
- **Agents**: 100 (default).
- **DNA**: Agents are initialized with random behavior:
  - **Builders**: Create sand (Radiate).
  - **Eaters**: Remove sand (Siphon).
- **Emergence**: Watch for "Sand Structures" built by agents, and how avalanches disrupt or transport the colonies.

## 🚀 Running

```bash
cargo run -p chimera-sandpile
```

## 🧬 Lineage

- **Parent A**: `experiments/chimera-lang` (The Genetic Engine)
- **Parent B**: `experiments/sandpile-scheduler` (The Physics Engine)
- **Novel Trait**: **Avalanche Surfing**. Movement driven by environmental instability.
