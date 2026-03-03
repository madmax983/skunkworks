# 🧫 `myco-fluid`

A biological-magnetic hybrid TUI experiment.

## Lineage

**Parents**:
- `experiments/myco-transit` (Slime mold cellular automata and pheromone pathfinding)
- `experiments/ferrous-fluid` (Magnetic particle simulation)

**Created by**: The Splice Surgeon 🧬

## Phenotype (Emergent Trait)

**Pheromone-Guided Magnetic Particles:**
The particles from `ferrous-fluid` act as the agents. They are primarily driven by raw magnetic forces (Pull/Repel from user-placed magnets), but as they move, they deposit a "pheromone trail" onto a grid (inherited from `myco-transit`). This trail diffuses and decays over time.

Crucially, the particles are not just passive writers to the grid—they *read* it. They calculate the gradient of the local pheromone density and are gently pulled towards areas of higher density.

This creates an emergent "Magnetic Memory". The trails formed by past magnetic lines of force become stable physical structures guiding future particles. It’s no longer just a fluid responding to magnets; it's a slime mold that reorganizes its pathing networks based on magnetic stimuli.

## Controls
- `[WASD]` Move Magnet
- `[Space]` Toggle Polarity (Red = North/Attract, Blue = South/Repel)
- `[Enter]` Add Magnet
- `[R]` Reset
- `[Q]` Quit
