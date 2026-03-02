# 🧲🦠 Gray-Fluid (Magnetic Reaction-Diffusion)

A TUI simulation crossing Gray-Scott reaction-diffusion with ferrous fluid particle dynamics.

## 🧬 Lineage

- **Parent A:** `crates/gray-scott` (Reaction-Diffusion System)
- **Parent B:** `experiments/ferrous-fluid` (Magnetic Particles)
- **Novel Trait:** Macroscopic-Microscopic Feedback Loop. The particles shape the chemical environment, while the chemical environment shapes the magnetic force.

## 🧪 Physics

The simulation runs a Gray-Scott chemical simulation alongside a particle engine. The emergent behavior comes from the following cross-pollination rules:
1. **Chemical Deposition:** Particles act as catalysts, depositing the 'V' chemical into the Gray-Scott grid at their locations.
2. **Magnetic Permeability:** The underlying 'U' chemical concentration acts as a magnetic lens multiplier. Areas with lower 'U' concentration multiply the pull (or push) of magnets.

## 🕹️ Controls

- **WASD**: Move the last active magnet.
- **Space**: Toggle polarity of the last magnet (North/South).
- **Enter**: Spawn a new magnet at the center.
- **R**: Reset the simulation.
- **Q**: Quit.
