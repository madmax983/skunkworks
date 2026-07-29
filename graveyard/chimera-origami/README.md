# Chimera Origami 🧬🦢

**A hybrid of `chimera-lang` and `origami`.**

"The paper breathes with computational intent."

## Concept

This experiment combines **Genetic Programming (ChimeraVM)** with a **Position Based Dynamics (PBD) soft-body mesh**.
It creates an emergent, self-actuating sheet of paper where the structural geometry is actively driven by internal computational agents.

- **Body**: A Miura-ori tessellation soft-body mesh.
- **Brain**: A swarm of minimal `ChimeraVM` genetic instances.
- **Actuation**: Each physical constraint in the soft-body mesh holds a `ChimeraVM`. The metabolic energy state of the VM dictates the physical contraction or relaxation of the constraint.

## Lineage

- **Parent A**: `experiments/chimera-lang` (The Genetic Code) - Provided the `ChimeraVM` architecture, `OpCode` logic, and metabolic energy tracking. The VMs act as the "muscles" or "neurons" embedded in the paper.
- **Parent B**: `crates/origami` (The Physical Medium) - Provided the `MiuraParams` generation, vertex structure, and PBD mesh topology.

## Emergent Behavior

- **Metabolic Folding**: The mesh does not fold randomly; it folds based on the internal computational states of the genetic algorithms running inside it. If an area of the paper gains high metabolic energy (simulated through the VM's state), it locally contracts, creating dynamic structural ripples across the surface.

## Controls

- **Mouse Drag**: Orbit the camera around the breathing mesh.

## Build

```bash
cargo run -p chimera-origami
```