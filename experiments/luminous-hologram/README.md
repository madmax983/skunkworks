# Luminous Hologram ✨✍️

**Lineage:** `hologram-text` × `luminous-flock`

A hybrid experiment by **The Splice Surgeon 🧬**.

## Concept
Swarming boids represented in a holographic frequency domain. Instead of physical positions, boids flock based on phase-coupling and their positions are reconstructed from their holographic interference patterns.

**Luminous Hologram** asks: *Can synchronous motion emerge within a dynamically shifting holographic projection?*

The system projects 3D text into a terminal frequency domain. The reconstruction (the target) acts as a series of strange attractors for the boid flock. The boids exhibit pulse-coupled synchronization as they flock towards these attractors to render the text dynamically.

## Genetics & Lineage
- **Parent A (hologram-text):** 2D Hologram encoding/decoding via FFT. Supplies the "strange attractors" for the spatial domain.
- **Parent B (luminous-flock):** Pulse-coupled boid oscillator synchronization. Supplies the agents and their interactions.
- **Emergent Trait:** Phase-Coupled Holographic Swarming. Boids sync their phases to form coherent glowing shapes from chaotic interference patterns, effectively "drawing" the text with glowing swarms.

## Controls
- **Arrow Keys:** Adjust the angle of holographic reconstruction. (-20, -10) brings the image into focus.
- **Typing:** Updates the text buffer being holographically encoded.
- **Enter:** Snap the reconstruction angle back to perfect focus.
- **Escape:** Quit the simulation.

## Technical Details
- **Physics:** Boids calculate flocking forces (separation, alignment, cohesion) and attractor forces toward the nearest reconstructed spatial pixel.
- **Rendering:** Uses a split-screen `ratatui` canvas. The left panel shows the raw frequency domain (Hologram), and the right panel shows the boids reconstructing the spatial domain.
