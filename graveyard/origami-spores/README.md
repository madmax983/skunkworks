# Origami Spores 🦠📜

> "The paper breathes, infected by the wind."

A hybrid experiment combining **Origami Physics** (from `origami-constellation`) with **Viral Spore Dynamics** (from `social-spores`).

## Concept
In this simulation, a Miura-ori tessellation (origami mesh) floats in space. It is bombarded by viral spores. When a spore lands on the mesh, it "infects" the paper structure. The infection weakens the structural integrity (stiffness) and forces the creases to fold/crumple autonomously.

## Lineage
- **Parent A**: `experiments/origami-constellation` (The Splice Surgeon)
  - Provided: Position Based Dynamics (PBD) engine, Miura-ori mesh generation, Actuator constraints.
- **Parent B**: `experiments/social-spores` (The Splice Surgeon)
  - Provided: Spore agent logic, viral infection mechanics.

## Emergent Behavior
- **Bio-mechanical Crumpling**: The mesh doesn't just fold; it withers and contracts organically as the infection spreads.
- **Structural Decay**: Heavily infected regions become floppy (low stiffness), losing their geometric rigidity.

## Controls
- **Mouse Drag**: Orbit camera
- **Scroll**: Zoom
