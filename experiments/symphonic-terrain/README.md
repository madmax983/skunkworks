# Symphonic Terrain 🏔️🎶

> "The landscape speaks, and the spheres sing its contours." - The Splice Surgeon

A hybrid experiment combining text-based terrain generation with orbital physics and audio synthesis.

## 🧬 Lineage

- **Parent A**: `experiments/text-terra` (The Typographer)
  - *Inherited Trait*: Procedural terrain generation from text glyphs.
- **Parent B**: `experiments/harmony-of-spheres` (The Astronomer)
  - *Inherited Trait*: N-body orbital physics and velocity-based audio modulation.

## ⚗️ The Hybrid

**Symphonic Terrain** creates a 3D landscape from a seed word (default: "SYMPHONY"). Orbiting "notes" (spheres) traverse this landscape.

- **Visuals**: The terrain height is determined by Perlin noise blended with the rasterized glyphs of the seed word.
- **Physics**: Spheres orbit a central attractor but are visually situated above the terrain.
- **Audio**: As spheres pass over the terrain, their audio frequency is modulated by the height of the ground beneath them. High peaks (text) create higher pitches. (Note: Audio requires `audio` feature and system dependencies).

## 🚀 Usage

```bash
cargo run -p symphonic-terrain
```

To enable audio (requires ALSA/system deps):
```bash
cargo run -p symphonic-terrain --features audio
```

## 🎮 Controls

- **Arrows**: Rotate Camera
- **W/S**: Zoom
- **Click**: Spawn a new orbiting note
