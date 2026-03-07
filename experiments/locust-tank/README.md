# Locust Tank

**Experiment Type:** Swarm Intelligence / Acoustic Wave Physics
**Status:** [FRESH]
**Stack:** Rust, Macroquad, Rayon, Resonance Audio

## 🧬 Lineage

- **Parent A:** `experiments/locust-ddos` (Swarm intelligence, packet foraging)
- **Parent B:** `experiments/ripple-tank` (2D physical modeling synthesis, wave propagation)

## Concept

Acoustic-Swarm Symbiosis. You can visually see and audibly hear the macroscopic swarm behavior translate into pressure waves, while the waves continuously restructure the swarm paths into resonant clusters. Instead of firewalls and targets, the swarm is bounded in a ripple tank grid. When swarm agents hit boundaries or each other, they pluck the ripple tank, generating acoustic waves that propagate. Conversely, the wave pressure field affects the swarm's movement, acting like acoustic levitation or repulsive force fields.

## Controls

- **Left Click:** Pluck the water surface manually.
- **Space:** Clear all waves.
- **C:** Clear all walls.
- **R:** Reset the simulation.
- **Q:** Quit.

## Technical Details

- **Locust Agents:** Agents are simulated over a grid bounding the acoustic wave propagation field.
- **Wave Tank:** Solves the 2D wave equation in real-time, feeding audio to the output stream.
- **Coupling:** Particles inject pressure upon events; pressure gradients compute forces back to the particles.
