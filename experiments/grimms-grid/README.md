# Grimm's Grid ⚛️📜

"Grimm's Grid" is a cellular automaton that simulates phonological evolution. Instead of dead/alive cells, each cell contains a phoneme (a unit of sound) defined by its features (Manner, Place, Voicing).

The rules of evolution are based on real-world linguistic phenomena such as Grimm's Law, Palatalization, and Lenition.

## Concept

Imagine a 2D surface where sound waves propagate. As phonemes interact with their neighbors, they influence each other. A voiceless stop between two vowels might become voiced (Lenition). A velar stop followed by a front vowel might shift forward (Palatalization).

This simulation explores how local interactions can lead to global patterns, much like how languages evolve over time and space.

## Controls

- **Space**: Pause / Resume the simulation.
- **n**: Step forward one generation (when paused).
- **r**: Reset the grid with a seed text ("THE QUICK BROWN FOX...").
- **s**: Reset the grid with random phonemes (Noise).
- **q**: Quit.

## Rules

The automaton applies the following probabilistic rules:

1.  **Assimilation**: A voiceless phoneme surrounded by voiced neighbors tends to become voiced.
2.  **Palatalization**: Velar or Alveolar stops followed by front vowels tend to become Palatal fricatives.
3.  **Lenition**: Intervocalic stops weaken to fricatives.
4.  **Epenthesis**: A vowel (schwa) may spontaneously appear between two consonants to break a cluster.
5.  **Cluster Simplification**: Dense consonant clusters may lose members.
6.  **Dialect Diffusion**: Vertical neighbors (representing parallel dialects) occasionally copy each other.

## Technical Details

- **Stack**: Rust, `ratatui`, `crossterm`.
- **Architecture**: Double-buffered cellular automaton.
- **State**: Each cell is an `Option<Phoneme>`, where `Phoneme` is a struct containing linguistic features.

## How to Run

```bash
cargo run --bin grimms-grid
```
