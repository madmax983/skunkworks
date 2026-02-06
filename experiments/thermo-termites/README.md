# Thermo-Termites ⚛️🐜

> "A single termite is blind, but the colony is a cooling engineer."

**Thermo-Termites** is an emergent simulation of termite-inspired agents building passive cooling structures ("fins") around heat-generating servers.

## The Concept

This experiment combines **Termite Mound Ventilation** with **Data Center Cooling**.
Agents (Termites) follow simple stigmergic rules to move material (walls/dirt) based on local heat gradients and structural density.

- **Servers**: Generate heat that diffuses through the grid.
- **Termites**:
    - Move randomly (Brownian motion).
    - Pick up dirt if it is isolated (clutter) or if the area is cold.
    - Drop dirt if the area is hot (building shields/fins) or adjacent to other walls (building continuous structures).
- **Emergence**: Over time, termites should redistribute the random dirt into structures that interact with the heat map.

## How to Run

```bash
cargo run -p thermo-termites
```

## Controls

- `q`: Quit the simulation.

## Legend

- **Red Block**: Server (Heat Source)
- **White Block**: Wall (Dirt/Material)
- **Blue/Green 't'**: Termite (Blue = Empty, Green = Carrying Dirt)
- **Background**: Heat Map (Black -> Orange -> Magenta)
