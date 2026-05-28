# Morph Physics

> "Phonology is just particle physics with meaning." - Genesis: The Philologist

**Morph Physics** is a simulation that treats linguistic phonemes as physical particles. It visualizes the evolution of words through sound laws, modeled as forces in a dynamic system.

## Concept

Each phoneme is a particle with 3 distinctive features:
1.  **Place** (Red Channel): 0.0 (Labial) -> 1.0 (Glottal)
2.  **Manner** (Green Channel): 0.0 (Stop) -> 1.0 (Vowel)
3.  **Voice** (Blue Channel): 0.0 (Unvoiced) -> 1.0 (Voiced)

Words are molecules—chains of phonemes connected by springs.

### Laws of Physics (Sound Laws)

*   **Springs**: Maintain the structural integrity of a word.
*   **Repulsion**: Phonemes naturally repel to maintain distinctness (Dispersion Theory).
*   **Lenition (Heat)**: Adding energy (velocity) causes particles to drift towards higher "Manner" values (Stops -> Fricatives -> Vowels).
*   **Assimilation**: (Planned) Particles attract neighbors with similar features.

## Controls

*   **[SPACE]**: Heat the system. Increases velocity and causes lenition (decay).
*   **[R]**: Reset / Spawn a new word ("mater").

## Running

```bash
cargo run -p morph-physics
```
