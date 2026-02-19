# Quantum Groove ⚛️🥁

> A Rhythmic Roguelike where quantum states collapse on the beat.

**Parent A:** `quantum-rogue` (Quantum Logic)
**Parent B:** `atomic-groove` (Rhythm + Audio)

## Concept
In `quantum-groove`, the dungeon exists in a superposition of states. The "Observer Effect" is tied to the rhythm of the universe (the beat).
- **On Beat:** Your movement is a valid measurement. You collapse the wave function of qubits you collide with.
- **Off Beat:** Your movement is "fuzzy". You pass through qubits without collapsing them (ghostly behavior), or your actions fail.

## How to Play
- **Controls:** Arrow Keys or `h` `j` `k` `l` to move. `q` to quit.
- **Objective:** Reach the Exit (`>`) while collecting/measuring Qubits (`Q` / `0` / `1`).
- **Mechanic:** Watch the metronome bar at the top. Move when the bar is full (Green).
  - **Hit:** If you hit a Qubit on the beat, it collapses to `|0>` (vanishes) or `|1>` (score!).
  - **Miss:** If you move off-beat, you are unstable.

## Audio
The game generates a `groove_output.wav` file in the current directory, recording the "song" of your run. The beat and your actions create a unique audio track.

## Lineage
- **Quantum Logic:** Inherited from `quantum-rogue`. Qubits, Gates (Hadamard), Measurement.
- **Rhythm Engine:** Inherited from `atomic-groove`. WAV generation, Beat timing, Audio Events.
- **Novelty:** The synchronization of Quantum Measurement with Musical Rhythm.
