# Luminous Limb 🦑✨

> "The tentacle reached out, and the stars scattered."

A genetic hybrid of `fabric-limb` (Inverse Kinematics) and `luminous-flock` (Boid Swarm).

## 🧬 Lineage

- **Parent A**: `experiments/fabric-limb` (The Code Dancer)
  - *Traits Inherited*: FABRIK Inverse Kinematics algorithm (`Arm` struct), joint constraints.
- **Parent B**: `experiments/luminous-flock`
  - *Traits Inherited*: Boid flocking rules (Separation, Alignment, Cohesion), Firefly synchronization (`Boid` struct).

## 🧪 Hybrid Vigor

The combination creates a predator-prey dynamic where the environment (the flock) is reactive.
- **Predator**: The multi-jointed arm chases the center of mass of the flock.
- **Prey**: The boids flock together but flee from the arm's end-effector.
- **Emergence**: The flock forms a dynamic "halo" around the predator, constantly shifting to maintain safety while trying to stay together.

## 🎮 Controls

- **`m`**: Toggle Mode (Auto/Manual)
  - **Auto**: Arm hunts the flock's center of mass.
  - **Manual**: Control the target with WASD.
- **`WASD`**: Move target (in Manual mode).
- **`q`**: Quit.

## 🔬 Implementation Details

- **Language**: Rust
- **Rendering**: `ratatui` (TUI Canvas)
- **Physics**: `locus` (Vector math), `flocking` (Swarm behaviors)
