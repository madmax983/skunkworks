# Sonar Swarm 🦇

**A Hybrid Experiment by The Splice Surgeon**

> "Echolocation is just touch at a distance."

## 🔬 Experiment Analysis

This experiment combines active sonar acoustics with flocking behavior.

- **Parent A**: `experiments/echo-chamber` (Acoustic FDTD Simulation)
- **Parent B**: `experiments/luminous-flock` (Flocking Agents)

### Concept
Agents ("Sonar Boids") navigate a dark environment using active echolocation. They emit sound pulses ("pings") into a simulated acoustic pressure field. The pressure waves reflect off walls and other objects. Boids sense the local pressure gradient and steer away from high-intensity sound (echoes/noise), effectively avoiding obstacles and each other without visual sight.

### Lineage & Genetics
- **Allele A (Environment)**: FDTD Wave Equation solver from `echo-chamber`.
- **Allele B (Behavior)**: Boid flocking logic from `luminous-flock`, adapted to respond to pressure gradients.
- **Emergent Trait**: "Blind" flocking where formation is dictated by sound interference patterns. Boids avoid the "loud" walls and the "loud" centers of the flock, spacing themselves out based on acoustic wavelengths.

## 🕹️ Controls

- **Q**: Quit
- **R**: Reset Walls (Clear to open space)
- **Mouse Left Click**: Toggle Walls (Draw/Erase obstacles)

## 📊 Technical Details

- **Physics**: 2D Finite Difference Time Domain (FDTD) wave solver runs on the audio thread (~44.1kHz) for high-frequency simulation.
- **Interaction**: Boids send `Pluck` commands to the audio thread. The audio thread sends `PressureSnapshot`s (60Hz) to the main thread for navigation.
- **Rendering**: `ratatui` Canvas visualizes the pressure field intensity (Blue/Cyan) and the boids.
