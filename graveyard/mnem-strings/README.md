# Mnem Strings 🎻 decay

> "A codebase decays not with a whisper, but with a discordant symphony."

**Mnem Strings** is a hybrid experiment born from splicing `mnem-rot` with `ferrous-strings`. It creates a chaotic, acoustic-magnetic manifestation of codebase entropy.

## Concept

This experiment models a repository's dependency graph as a physical force field interacting with magnetic, vibrating strings.

*   **The Decaying Graph (`mnem-rot` allele):** Files are represented as nodes. Over time, simulated entropy ("code rot") increases, causing nodes to decay. High-entropy (rotting) nodes physically jitter and turn red.
*   **The Acoustic-Magnetic Strings (`ferrous-strings` allele):** A series of vibrating strings span the visual space, representing the fundamental structural integrity of the project.
*   **The Emergent Phenotype:** As nodes decay, they become unstable and violently pluck the structural strings. The amount of "rot" is translated directly into acoustic discord. A healthy codebase is silent. A decaying one is a cacophony of striking strings.

## Usage

```bash
cargo run -p mnem-strings
```

### Controls

*   **Right Click + Drag:** Pan camera.
*   **Scroll:** Zoom in/out.
*   **Hover Node:** "Refactor" / Heal a decaying node (silences it temporarily).

## The Code

*   `src/graph.rs`: Parses the local rust codebase into a node-edge graph.
*   `src/glitch.rs`: Text corruption logic for the UI overlay.
*   `src/string.rs`: The FerrousString physics entity (inherited from `ferrous-strings`).
*   `src/audio.rs`: The Karplus-Strong string synthesis engine.
*   `src/main.rs`: The core loop. Handles graph physics, decay, and the bidirectional coupling where rotting nodes pluck strings.

## Surgeon's Notes 🧬

The emergence here is beautiful and terrifying. Instead of passively watching a codebase turn red as entropy takes hold, you *hear* it falling apart. The physical impact of a rotting graph node striking a simulated magnetic string provides visceral feedback for technical debt.
