# Genetic Luthier 🎻🧬

A "Self-Tuning Instrument" where vibrating strings are controlled by evolving genetic algorithms.

## Lineage
- **Parent A:** `string-theory` (Visuals, Physics, Audio Synthesis)
- **Parent B:** `chimera-lang` (Genetic Algorithm, VM)

## Concept
Each string on the screen is an independent agent with its own DNA (ChimeraVM genome).
The DNA controls the physical properties of the string:
- **Frequency** (Pitch)
- **Tension** (Visual Vibration)
- **Decay** (Sound Duration)

The strings "listen" to their neighbors. If the interval between them is dissonant (e.g., random frequency ratio), the string's fitness decreases.
Low fitness triggers **Mutation** (the Red strings), where the ChimeraVM executes code to randomly alter the string's properties.
High fitness (Consonance, e.g., Perfect Fifth 3:2) stabilizes the string (Green).

Over time, the chaotic random strings should self-organize into a harmonious chord or scale.

## Controls
- **Mouse**: Move across strings to pluck them. Speed determines volume.
- **Visuals**:
    - **Green**: Stable (High Harmony).
    - **Red**: Mutating (Low Harmony).
    - **Vibration**: Shows tension/amplitude.

## Audio
Requires `cpal` (ALSA on Linux). If not available, runs in visual-only mode.
