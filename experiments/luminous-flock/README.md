# Luminous Flock ✨

**A Hybrid Experiment by The Splice Surgeon**

> "Bioluminescent synchronization in a moving coordinate system."

## 🔬 Experiment Analysis

This experiment combines flocking behavior with pulse-coupled oscillator synchronization.

- **Parent A**: `experiments/literary-boids` (Flocking Behavior)
- **Parent B**: `experiments/firefly-synapse` (Kuramoto/Firefly Synchronization)

### Concept
Standard firefly simulations (Kuramoto model) assume static agents. Boid simulations assume visual flocking without internal state synchronization.
**Luminous Flock** asks: *How does physical clustering affect temporal synchronization?*

Agents ("Luminous Boids") move according to separation, alignment, and cohesion rules. Simultaneously, they maintain an internal "phase" clock. When they flash, they nudge the phase of nearby neighbors.

### Lineage & Genetics
- **Allele A (Motion)**: Flocking physics from `literary-boids` (Reynolds' rules).
- **Allele B (Time)**: Pulse-coupled synchronization from `firefly-synapse`.
- **Emergent Trait**: Waves of light that propagate through moving clusters. Synchronization becomes localized to physical flocks.

## 🕹️ Controls

- **Q**: Quit
- **R**: Reset simulation

## 📊 Technical Details

- **Physics**: Combined O(N^2) loop for spatial forces and phase coupling.
- **Rendering**: `ratatui` Canvas.
- **DNA**: Each boid has randomized traits (speed, view radius, coupling strength, color).
