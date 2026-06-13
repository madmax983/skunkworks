# Ferrous Fold 🧲🦢

A hybrid experiment combining **Magnetic Soft Body Physics** (Ferrous Tissue) with **Procedural Origami** (Miura-ori).

## 🧬 Lineage

- **Parent A**: `experiments/ferrous-tissue` (Magnetic Soft Body Physics, ChimeraVM)
- **Parent B**: `crates/origami` (Miura-ori Mesh Generation)

## 🔬 Concept

This experiment simulates a "Magneto-Mechanical Metamaterial".
- The body is a Miura-ori fold pattern generated procedurally.
- Each vertex is a magnetic particle simulated with Position Based Dynamics (PBD).
- **ChimeraVM** agents inhabit each vertex, sensing local magnetic fields and strain, and altering their own magnetic polarity.
- The entire sheet folds and unfolds autonomously as the magnetic domains shift.

## ✨ Novel Traits

- **Magneto-Mechanical Folding**: The fold angle is not actuated by motors, but by the collective magnetic field of the vertices.
- **Programmable Matter**: The folding logic is encoded in the DNA of the agents.
- **Self-Healing**: PBD constraints ensure the sheet returns to its connectivity even after extreme deformation.

## 🕹️ Controls

- **Arrows**: Rotate Camera
- **Z/X**: Zoom In/Out

## 🧠 Genetic Logic

The default DNA implements a simple magnetic oscillator:
1. **Target Magnetism** = 100 - Current Magnetism.
2. This causes poles to flip-flop, creating dynamic waves of folding/unfolding.
