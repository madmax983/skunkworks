# Struct Harmonics 🎼

**Lineage:** `struct-soup` × `orbital-harmonics`

A "Splice Surgeon" hybrid that visualizes the codebase as a gravitational system and sonifies its structure.

## Concept

Code structures (Structs and Enums) are physical bodies in a 2D space.
- **Mass:** Determined by the number of fields (dependencies).
- **Gravity:** Connected nodes (dependencies) attract each other.
- **Sound:** Each node has a resonant frequency based on its mass. Large, complex structs hum with low frequencies; small, light structs ping with high frequencies.

## Controls

- **WASD / Arrows:** Move the "Listener" (Camera)
- **+ / -:** Zoom In/Out
- **Space:** Pause/Resume Physics
- **h / ?:** Toggle Help
- **q / Esc:** Quit

## Emergent Behavior

The "sound" of the codebase emerges from the spatial arrangement. Clusters of tightly coupled code create a complex drone. As you navigate the code, you "hear" the architecture.

## Installation & Running

```bash
cargo run -p struct-harmonics -- <path-to-rust-workspace>
```

To enable audio (requires system ALSA libraries):
```bash
cargo run -p struct-harmonics --features audio -- <path-to-rust-workspace>
```
