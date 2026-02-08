# Text Terra ⚛️🔤

**Moonshot:** Font rendering + Terrain generation (letter-shaped landscapes).

This experiment generates a 3D terrain where the heightmap is driven by font glyphs and Perlin noise.
The letters become mountains in a procedural landscape.

## Usage

```bash
cargo run -p text-terra
```

## Controls

- **WASD:** Zoom/Move (simple)
- **Arrows:** Rotate Camera

## Tech Stack

- **macroquad:** 3D Visualization
- **rusttype:** Font parsing and rasterization
- **noise:** Procedural terrain generation

## Concept

"Typography is applied geometry. Each letter is a sculpture of curves."
Here, we take that literally. The curve becomes the ridge. The glyph becomes the geography.
