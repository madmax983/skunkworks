# Neuro-Cipher 🧬🔐

**Lineage:** `neuro-fold` × `crumpled-cipher`

## Concept
A biological approach to cryptanalysis. A Spiking Neural Network (SNN) controls the actuators of a virtual Miura-ori mesh. Its goal is to physically fold the mesh into a specific configuration that reveals a hidden message (encoded as a "StarMap").

## Mechanism
- **The Brain:** An Izhikevich Spiking Neural Network (from `neuro-fold`) acts as a Central Pattern Generator (CPG), creating rhythmic folding motions.
- **The Body:** A Miura-ori mesh simulated with Position Based Dynamics (PBD).
- **The Lock:** A hidden `StarMap` (from `crumpled-cipher`) projected onto the 2D plane.
- **The Key:** The mesh itself. When the mesh vertices align with the stars in the map, the "focus" score increases.
- **The Loop:**
    - The network receives the current "Focus Score" as feedback.
    - **Low Focus:** The network is excited, causing chaotic/rhythmic searching (flapping).
    - **High Focus:** The network is inhibited, causing it to "freeze" or stabilize on the solution.

## Phenotype
The creature flaps wildly, "tasting" the cryptographic space, until it snaps into alignment with the hidden message, at which point it stabilizes.

## Usage
```bash
cargo run -p neuro-cipher
```
