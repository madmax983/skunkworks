# Sonata Cycle 🎼⚙️

**A Clockwork CPU that Sings.**

> "The instructions are notes, the program is a melody, and the execution is a symphony."

This experiment visualizes a CPU where the program is stored on a rotating music box cylinder.
As the cylinder rotates, pins trigger logic levers which update the CPU state (Accumulator, PC) and play a musical note corresponding to the instruction.

## Mechanism

- **Verge Escapement**: A physics-based escapement (simulated via `bevy_rapier2d`) drives the main shaft.
- **Pin Barrel**: A rotating drum with pins representing the program.
- **Logic Levers**: Physical levers that act as the "Read Head". When struck, they trigger an instruction.
- **Audio Synthesis**: Procedural sine wave synthesis generates notes based on the instruction type (Pentatonic scale).

## Controls

- The simulation runs automatically.
- Watch the "CPU State" update as the pins hit the lever.
- Listen to the "Sonata" of the code.

## Tech Stack

- **Bevy**: Game Engine (ECS).
- **Bevy Rapier 2D**: Physics simulation.
- **Bevy Prototype Lyon**: Vector shape rendering.
- **Bevy Audio**: Sound playback (Procedural WAV generation).

## Building

To enable audio (requires ALSA on Linux):
```bash
cargo run --features audio
```

By default, audio is disabled to ensure CI compatibility.
