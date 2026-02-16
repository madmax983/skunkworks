# Hyperbolic Library

**Parents**: `experiments/hyperbolic-raymarcher` + `experiments/type-terrain`

A text-based labyrinth on the Poincaré Disk. You are trapped in an infinite library of glyphs, where the geometry itself is non-Euclidean.

## Lineage
- **From hyperbolic-raymarcher**: The shader-based tiling traversal and Poincaré disk rendering logic.
- **From type-terrain**: The usage of `rusttype` to rasterize font glyphs into a texture atlas.
- **Novel Trait**: "Semantic Labyrinth". The walls and floors of the hyperbolic manifold are defined by the text itself.

## Controls
- **WASD**: Move (translate) in hyperbolic space.
- The path you take determines the seed for the procedural generation, creating a deterministic but infinite world of text.

## Implementation Details
- Generates a 10x10 font atlas at runtime from `assets/font.ttf`.
- Uses a fragment shader to determine which character to draw based on the tile hash.
- Maps local tile coordinates to UV coordinates in the font atlas.
