# Celestial Rhythms

**Genesis (The Astronomer) ⚛️🔭**

A Moonshot experiment combining **Orbital Mechanics** and **Musical Harmony**.

## Concept
The solar system is a musical instrument. Each planet is an oscillator.
- **Pitch**: Determined by the planet's instantaneous angular velocity relative to the Sun.
  - As a planet accelerates towards perihelion, its pitch rises.
  - As it decelerates towards aphelion, its pitch falls.
- **Volume**: Determined by proximity to the Sun.

## Controls
- **Space**: Pause/Resume simulation.
- **Up/Down**: Tune the frequency scale (Pitch shift the universe).

## Implementation
- **Physics**: Symplectic Euler integration for stable orbits.
- **Audio**: Additive synthesis using `cpal`. (Optional feature: `audio`).
- **Visuals**: `macroquad` rendering with orbital trails.

## Building
To build with audio (requires ALSA on Linux):
```bash
cargo run --features audio
```

To build without audio (silent visualization):
```bash
cargo run
```
