# Syntax Fugue 🎼⚛️

> "Code is not just logic. It is structure, rhythm, and harmony." - Genesis

**Syntax Fugue** is a synesthetic translator that turns Rust source code into a musical fugue.
It analyzes the Abstract Syntax Tree (AST) and maps code structures to musical voices, creating a polyphonic composition where the "melody" is the logic and the "harmony" is the data structure.

## The Orchestra

The code is divided into four voices, mimicking a baroque fugue:

*   **Bass (Structs)**: The foundation. Deep, long sine waves representing the data structures.
*   **Tenor (Enums)**: The rhythm. Fast, square-wave ostinatos representing state variants.
*   **Alto (Impls)**: The counter-melody. Arpeggiated triangle waves representing trait implementations.
*   **Soprano (Functions)**: The melody. Clear, singing sine waves representing the execution flow.

## The Form

The composition follows a staggered entry (Fugue):
1.  **Bass** enters immediately (0s).
2.  **Tenor** enters at 4s.
3.  **Alto** enters at 8s.
4.  **Soprano** enters at 12s.

## Usage

Run the visualization (silent or with offline WAV generation):

```bash
cargo run -p syntax-fugue -- [file_path]
```

Example:
```bash
cargo run -p syntax-fugue -- experiments/syntax-fugue/src/parser.rs
```

### Real-time Audio

To enable real-time audio playback (requires `alsa` on Linux):

```bash
cargo run -p syntax-fugue --features audio
```

The output is also saved to `output.wav` upon exit (press `q`).

## Concept

This project explores the prompt: **"Source code structure + Musical form (sonata, fugue)"**.
Instead of just mapping text to sound, we map the *grammatical structure* of the code to the *harmonic structure* of the music.
