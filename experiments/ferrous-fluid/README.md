# 🧲 Ferrous Fluid

A TUI simulation of Ferrofluid dynamics using magnetic forces and grid-based fluid pressure.

## 🧬 Lineage

- **Parent A:** `experiments/ferrous-graph` (Magnetic physics, Graph layout)
- **Parent B:** `experiments/fluid-rain` (Fluid particles, Gravity)
- **Novel Trait:** Grid-based density pressure combined with inverse-square magnetic attraction creates "spiking" and clustering behaviors in a text-based medium.

## 🕹️ Controls

- **WASD**: Move the last active magnet.
- **Space**: Toggle polarity of the last magnet (North/South).
- **Enter**: Spawn a new magnet at the center.
- **R**: Reset the simulation.
- **Q**: Quit.

## 🧪 Physics

The simulation uses a hybrid Lagrangian-Eulerian approach:
1. **Particles (Lagrangian):** Move freely, affected by gravity and magnetism.
2. **Grid (Eulerian):** Calculates density fields from particle positions.
3. **Pressure Gradient:** Particles are pushed away from high-density grid cells to simulate fluid volume and incompressibility.
