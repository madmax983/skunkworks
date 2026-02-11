# Chimera-Fold 🧬📄

**Lineage:** `chimera-lang` × `neuro-fold`

## Concept
A biological experiment where the "body" is a Miura-ori origami mesh and the "brain" is a distributed cluster of ChimeraVMs. Each column of the mesh is controlled by a separate VM instance.

## Traits
- **Origami Genetics**: The behavior of the mesh (folding/unfolding) is determined by the `Dna` executed by the VMs.
- **Distributed Intelligence**: Multiple VMs run in parallel, coordinating (or competing) to move the body.
- **Proprioception**: The VMs can read the physical strain of the mesh directly into their memory.

## Architecture
- **PBD Physics**: Position Based Dynamics simulation of the mesh (inherited from `neuro-fold`).
- **ChimeraVM**: Stack-based genetic virtual machine (inherited from `chimera-lang`).
- **Input/Output**:
    - **Input**: Strain -> `Grid[0][0]`
    - **Clock**: Time -> `Grid[0][3]`
    - **ID**: Brain ID -> `Grid[0][2]`
    - **Output**: `Grid[0][1]` -> Actuator Target Factor

## Running
```bash
cargo run -p chimera-fold
```
