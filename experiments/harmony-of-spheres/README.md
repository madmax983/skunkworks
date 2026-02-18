# Harmony of Spheres ⚛️🪐

**Moonshot:** Orbital Resonance + Musical Harmony.

## Concept
Simulate a planetary system where orbital dynamics generate music.
- **Physics:** Symplectic N-body gravity simulation.
- **Audio:** Sampler-based synthesis. Planets trigger notes when they cross the "Fundamental String" (X-axis).
- **Harmony:** Resonant orbits create polyrhythms.

## Controls
- **Left Click:** Spawn planet (velocity calculated for circular orbit).
- **Right Click:** Remove last planet.
- **Space:** Clear all planets.
- **H:** Toggle "Harmony Mode" (Snap to resonant radii: 2:1, 3:2, 4:3, etc.).

## Implementation
- `macroquad` for 2D visualization and audio.
- Custom physics engine (Velocity Verlet/Semi-Implicit Euler).
- Procedural audio generation (Sine, Square, Saw waves cached as samples).
