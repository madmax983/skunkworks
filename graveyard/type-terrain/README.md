# Type Terrain ⚛️🔤

**Genesis: The Typographer**

> Walking through the valley of the shadow of the serif.

`type-terrain` is a moonshot experiment that procedurally generates 3D landscapes from font glyphs. It treats the coverage of a rasterized glyph as a heightmap, applies a distance transform (simulated via blur) to create slopes, and mixes it with Perlin noise to add texture.

## Concept

Typography is usually 2D. What if it had altitude?
*   **Glyphs** become mountains.
*   **Counters** become lakes.
*   **Serifs** become ridges.

## Controls

*   **WASD**: Move camera (Fly).
*   **Q/E**: Up/Down.
*   **Arrow Keys**: Look around.

## Technical Details

*   **Engine**: `macroquad`
*   **Font Parsing**: `rusttype`
*   **Noise**: `noise-rs` (Perlin)
*   **Rendering**: 3D Mesh generation from heightmap.

## How to Run

```bash
cargo run -p type-terrain
```

Ensure you run this from the workspace root or the `experiments/type-terrain` directory.
