# Crumpled Memory 🦢🧠

**Lineage:** `mnemosyne` × `origami-terrain`

A hybrid experiment exploring "Mechanical Erasure" of digital memory.

## Concept
A digital image ("Memory") is mapped onto a Miura-ori fold structure. As the user physically manipulates the structure (folding/unfolding), the mechanical stress at the crease lines causes the data to decay.

The more you fold the memory to store it, the more it is destroyed.

## Biological/Mechanical Traits
- **Mnemosyne Allele**: Memory decay logic, image texture handling, erosion algorithms.
- **Origami Allele**: Miura-ori mesh generation, folding kinematics.
- **Novel Trait**: **Mechanical Erasure**. The integrity of the data is coupled to the physical state of the medium.

## Controls
- **UP / DOWN**: Fold / Unfold the memory cloth.
- **Mouse Drag**: Rotate camera.
- **Mouse Scroll**: Zoom.

## Implementation Details
- Uses `macroquad` for rendering.
- Uses `origami` crate (from `origami-terrain`) for mesh generation.
- Stress is calculated based on the fold angle (extension) and applied to the underlying image buffer.
- Bresenham's line algorithm maps the 3D fold lines to the 2D texture space to apply damage.
