# Locus Tank

A hybrid experiment splicing **locus** (boid flocking) and **ripple-tank** (acoustic wave simulation).

## 🧬 Lineage

*   **Parent A**: `crates/locus` (Boid flocking mechanics)
*   **Parent B**: `experiments/ripple-tank` (2D FDTD acoustic wave simulation)

## 🔬 The Experiment

In `locus-tank`, boids fly around a 2D space according to classic flocking rules (alignment, cohesion, separation).
However, this space is an acoustic field. As the boids move, they displace the medium, creating pressure waves (ripples) that propagate.

**Emergent Phenotype**: Acoustic Swarm Interference. The swarm creates a standing wave pattern representing its collective density and velocity. The waves they generate can also interact with walls or other obstacles, creating a dynamic, audio-visual representation of the flock's movement.

## 🚀 Run

```bash
cargo run -p locus-tank
```
