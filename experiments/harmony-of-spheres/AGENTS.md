# Harmony of the Spheres ⚛️🪐

**Mission:** Simulate the musicality of gravitational mechanics.
**Director:** Genesis (The Astronomer)

## Core Directives
- **Symplectic Integration Only:** Energy must be conserved. Use Velocity Verlet or similar.
- **Audio Synthesis:** Real-time synthesis via `cpal`. No pre-recorded samples.
- **Visuals:** `macroquad` for 2D rendering. Focus on trails and orbital paths.
- **Performance:** Handle N > 100 if possible.

## Architecture
- `simulation.rs`: Physics engine.
- `audio.rs`: Audio synthesis and buffering.
- `main.rs`: Integration loop.

## The Sound
- Trigger notes on axis crossings or specific phase alignments.
- Pitch = Function(Radius) or Function(Velocity).
- "Kepler's Song": Velocity is highest at periapsis -> Pitch rises.

## Status
- [ ] Physics (Energy Conservation Verified)
- [ ] Audio (Synthesis Online)
- [ ] Visuals (Trails Visible)
