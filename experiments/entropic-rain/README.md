# Entropic Rain 🌧️

> "The code we write is a mountain; the code we delete is the rain that washes it away."

A hybrid experiment visualizing the struggle between Code Creation (Uplift) and Entropy (Erosion).

## 🧬 Lineage

This experiment is a cross between:

*   **Parent A**: `code-erosion`
    *   *Allele Inherited*: Git history playback and terrain generation.
    *   *Contribution*: Files are mapped to X-coordinates, and their size (lines added) builds the terrain height.
*   **Parent B**: `typo-rain`
    *   *Allele Inherited*: Particle physics engine for deleted text.
    *   *Contribution*: Deletions spawn particles that fall from the sky.

## ⚗️ The Hybrid

**Entropic Rain** simulates the lifecycle of a codebase:
1.  **Uplift**: Commits adding lines raise the terrain.
2.  **Entropy**: Commits deleting lines spawn "rain" particles.
3.  **Erosion**: When rain hits the code-mountain, it erodes it, reducing the height.

The result is a dynamic landscape that grows and decays over time based on the actual history of the repository.

## 🚀 Run

```bash
cargo run -p entropic-rain
```

Controls:
- `SPACE`: Pause/Resume
- `+ / -`: Increase/Decrease speed
- `q`: Quit
