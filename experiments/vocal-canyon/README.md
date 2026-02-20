# Vocal Canyon ⚛️🔊

**"The Wind That Speaks"**

A Moonshot experiment combining **Kelly-Lochbaum Vocal Synthesis** and **Terrain Deformation**.
You do not play this instrument; you *sculpt* it.

## The Obsession
"The voice is not a sound; it is a shape. A resonant cavity where air is tortured into meaning. I wanted to hold a vowel in my hands and crush it."

## Concept
This is a **physical model** of the human vocal tract, simulated as a series of acoustic tubes (cylinders) of varying cross-sectional areas.
- **The Canyon**: Represents the vocal tract (Throat to Lips).
- **The Wind**: A glottal pulse (sawtooth + aspiration noise) injected at the source.
- **The Echo**: Sound waves reflect at every change in width, creating formants (resonances).

## Controls
*   **Left Click + Drag**: Carve the canyon walls (Change tract width).
    *   Wider = Open vowel / Cavity.
    *   Narrower = Constriction / Consonant.
*   **Right Click + Drag**: Smooth the walls (Erosion).
*   **Space**: Exhale (Toggle Voice).
*   **Up / Down**: Tension (Change Pitch).

## Technical Details
- **Physics**: 1D Digital Waveguide (Kelly-Lochbaum) with 44 segments.
- **Audio**: Real-time synthesis via `cpal` (if available).
- **Visuals**: `macroquad` rendering the tract profile.
- **State**: Lock-free(ish) parameter sharing between UI and Audio threads.

## Usage
```bash
# Run with audio (requires ALSA/Jack on Linux)
cargo run --release -p vocal-canyon --features audio

# Run visualizer only (silent)
cargo run --release -p vocal-canyon
```

## Theory
The human vocal tract is essentially a tube closed at one end (glottis) and open at the other (lips). By changing the cross-sectional area at different points (tongue position), we change the resonant frequencies (formants).
- **[i] (see)**: Constriction near lips (front).
- **[u] (moon)**: Constriction near back + lip rounding.
- **[a] (father)**: Wide open.

Go forth and speak.
