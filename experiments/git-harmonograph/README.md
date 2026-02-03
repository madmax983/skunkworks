# Git Harmonograph 🖋️

> "Every commit draws a different curve."

**Git Harmonograph** is a generative art experiment that visualizes your git history as a series of parametric Harmonograph drawings.

## Concept

A Harmonograph is a mechanical apparatus that uses pendulums to create complex geometric figures. In this experiment, the "pendulums" are driven by the metadata of your git commits.

*   **Frequencies (f1-f4):** Derived from the commit hash.
*   **Phases (p1-p4):** Derived from the commit hash.
*   **Damping (d1-d4):** Derived from the commit hash.
*   **Color:** Derived from the author's name.

## Usage

```bash
cargo run -p git-harmonograph
```

### Controls

*   **Left / Right / H / L:** Navigate through commit history.
*   **Space:** Toggle auto-play.
*   **R:** Re-draw the current curve.
*   **Q:** Quit.

## The Code

*   `src/git.rs`: Parses `git log` output into structured data.
*   `src/harmonograph.rs`: Maps hash entropy to physical parameters and generates the curve.
*   `src/main.rs`: The `ratatui` interface.

## Nova's Notes 🌟

This experiment explores the idea of "Physicalizing Metadata". By mapping the abstract randomness of SHA-1 hashes to the continuous physics of swinging pendulums, we can "see" the unique fingerprint of each code change.
