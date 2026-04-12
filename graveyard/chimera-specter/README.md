# 🧬 Chimera Specter

> "In the beginning was the Word, and the Word was Sound, and the Sound became Flesh."

**Chimera Specter** is a hybrid experiment combining `chimera-lang` (Genetic Programming) with `fluid-specter` (Audio Visualization).

It simulates a "Spectral Ecology" where:
1.  **The Environment** is a fluid simulation driven by audio input (microphone or simulated).
2.  **The Food** is the energy present in specific frequency bands of the audio spectrum.
3.  **The Agents** are ChimeraVM organisms. Their DNA determines which frequencies they can metabolize.

## Lineage

*   **Parent A**: `experiments/chimera-lang` (Biology/Logic)
    *   Contributed: `ChimeraVM`, Genetic Code structure.
*   **Parent B**: `experiments/fluid-specter` (Physics/Visualization)
    *   Contributed: Fluid Dynamics Solver, Audio FFT Analysis.
*   **Novel Trait**: **Spectral Metabolism**. Survival of the fittest is determined by the acoustic environment. Agents evolve to occupy the frequency niches available in the room.

## Usage

```bash
cargo run -p chimera-specter
```

Make some noise! Whistle, clap, or play music.
-   **Low Pitch (Bass)** feeds red agents.
-   **High Pitch (Treble)** feeds blue agents.
-   **Silence** leads to starvation.

## Controls

-   **Mouse Left Click**: Agitate the fluid manually.
