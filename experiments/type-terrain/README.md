# Type Terrain ⚛️🔤

**Genesis: The Typographer**

> "Typography is applied geometry. Each letter is a sculpture of curves."

`type-terrain` is a moonshot experiment that procedurally generates 3D landscapes from font glyphs using Squared Euclidean Distance Transforms (SEDT).

## Concept

Letters are not flat. They are islands in an ocean of ink.
*   **Glyphs** are rasterized into a binary grid.
*   **SDF Generation**: We compute the distance from every point inside the glyph to the nearest edge.
*   **Terrain**: This distance becomes altitude.
*   **Noise**: Perlin noise is added to the slopes to create rugged "letter-mountains".
*   **Water**: A water plane is added to simulate the sea level.

## Controls

*   **WASD**: Move camera (Fly).
*   **Shift**: Sprint (Move faster).
*   **Q/E**: Up/Down.
*   **Arrow Keys**: Look around.

## Technical Details

*   **Engine**: `macroquad`
*   **Font Parsing**: `rusttype`
*   **SDF**: Custom implementation of Squared Euclidean Distance Transform.
*   **Noise**: `noise-rs` (Perlin).
*   **Rendering**: 3D Mesh generation with vertex colors based on altitude (Sand, Grass, Rock, Snow).

## How to Run

```bash
cargo run -p type-terrain
```

Ensure you have a `font.ttf` in `assets/` or `experiments/type-terrain/assets/`.
