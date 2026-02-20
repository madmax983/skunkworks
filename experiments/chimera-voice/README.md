# Chimera Voice 🧬🗣️

**Parent A:** `experiments/chimera-lang` (Biological VM)
**Parent B:** `experiments/vocal-canyon` (Vocal Tract Simulation)

A "Living Synthesizer" where the parameters of a Kelly-Lochbaum vocal tract model are controlled by a population of genetic programs (ChimeraVM organisms).

## Concept

The **Grid is the Tongue**. The 16x16 memory grid of the ChimeraVM is mapped to the cross-sectional areas of the vocal tract.
- **X-Axis:** Position along the tract (Glottis -> Lips).
- **Y-Axis:** The "mass" or aperture at that segment.

As the organism lives, moves, and mutates on the grid, it changes the shape of the vocal tract, producing evolving phonemes and glitched speech.

## Usage

```bash
cargo run -p chimera-voice
```

**Controls:**
- **Space:** Reset the organism (New random genome).

## Lineage

- **Audio Engine:** Ported from `vocal-canyon` (Kelly-Lochbaum model).
- **Logic:** `chimera-lang` VM instances.
- **Visualization:** `macroquad` (inherited from `vocal-canyon`).

## Emergent Behavior

The organism evolves to "speak". High energy states correlate to higher pitch. Chaos mode triggers rapid phoneme shifting.
