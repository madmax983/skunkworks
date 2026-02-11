# Chimera Fold 🧬🦢

**Hybrid**: `chimera-lang` × `neuro-fold`

A distributed network of Chimera Virtual Machines controlling a physically simulated Miura-ori origami mesh.

## Concept

Each column of the Miura-ori mesh is controlled by an independent `ChimeraVM` instance. The VM acts as a "neuron" or local controller in a distributed nervous system.

- **Sensory Input**: The VM receives the average strain (extension/compression) of the vertical creases in its column. This is injected into grid cell `(0,0)`.
- **Motor Output**: The VM executes a genetic program to compute an actuation factor. This is read from grid cell `(0,1)` and applied to the mesh actuators.
- **Physics**: The mesh is simulated using Position Based Dynamics (PBD), allowing for realistic folding behavior.

## The Genome

The initial population runs a simple feedback loop:
1. Read Input (Strain)
2. Multiply by Gain
3. Write Output (Actuation)
4. Loop

This creates a reflexive behavior where the mesh resists deformation or amplifies it, depending on the gain.

## Controls

- **Mouse Click**: Excites the first VM (pacemaker) with a strong signal, triggering a wave of actuation.
- **Visuals**:
    - **Mesh**: Rendered in 3D.
    - **VM State**: A row of indicators shows the current output of each VM.

## Lineage

- **Parent A (`chimera-lang`)**: Provided the Virtual Machine architecture, genetic programming capability, and opcode set.
- **Parent B (`neuro-fold`)**: Provided the Miura-ori mesh generation, PBD physics engine, and actuator constraint logic.

## Emergent Behavior

The system exhibits peristaltic motion when excited, resembling a biological organism (e.g., a caterpillar or worm) moving through coordinated muscle contractions.
