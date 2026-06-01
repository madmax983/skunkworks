# mnem-flock 🧬

**Parents:** `experiments/mnem-rot` × `experiments/luminous-flock`

> "The codebase is decaying, but the swarm is here to heal."

## Concept

This experiment simulates a codebase as a graph of nodes (files) that naturally increase in **entropy** (rot) over time. As entropy rises, the text representation of the nodes glitches and becomes unreadable.

To combat this, a swarm of **Repair Drones** (boids) patrols the system. These drones exhibit flocking behavior (Separation, Alignment, Cohesion) and synchronization (Firefly flashing), but they possess a novel trait: **Taxis towards Decay**.

## Emergent Behavior

- **Rot:** Nodes slowly lose health, turning from Green -> Yellow -> Red. Their names become corrupted text.
- **Swarm:** The flock moves organically through the space.
- **Repair:** When a node's health drops below a threshold, nearby drones break formation to swarm the dying node.
- **Healing:** As drones gather around a node, they "heal" it (reduce entropy), restoring the text and color.
- **Pulse:** The drones synchronize their flashing, creating a visual rhythm of maintenance.

## Lineage

| Parent | Trait Inherited | Adaptation |
|--------|----------------|------------|
| `mnem-rot` | Graph Structure, Text Glitching, Entropy Logic | Ported to Macroquad for smoother animation. |
| `luminous-flock` | Boid Physics, Firefly Sync | Adapted to seek "Rot" as a target (Attraction force). |

## Controls

- **Automatic:** Sit back and watch the ecosystem maintain itself.
- **Visuals:**
  - **Green Nodes:** Healthy.
  - **Red Nodes:** Critical / Rotting.
  - **White Flashes:** Drone activity / Healing.

## Running

```bash
cargo run -p mnem-flock
```
