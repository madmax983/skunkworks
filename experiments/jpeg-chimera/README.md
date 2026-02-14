# JPEG Chimera 🧬🎞️

> "The artifact is the organism."

**JPEG Chimera** is a hybrid experiment combining the "bit rot" simulation of `jpeg-garden` with the evolving genetic organisms of `chimera-lang`.

In this simulation, ChimeraVM agents inhabit the frequency domain of a JPEG image. They live inside the Discrete Cosine Transform (DCT) coefficients.

## Concept

-   **Environment**: A 2D grid of DCT blocks.
-   **Agents**: ChimeraVM instances that "possess" a 16x16 pixel area (four 8x8 DCT blocks).
-   **Interaction**:
    -   **GRead**: Agents read the DCT coefficients (Y channel) as their local memory grid.
    -   **GWrite**: Agents modify the coefficients, causing compression artifacts, noise, or "glitches" in the reconstructed image.
    -   **Evolution**: Agents mutate their code (DNA) over time, learning to manipulate the image frequencies to survive (or just create chaos).

## Lineage

-   **Parent A**: `experiments/jpeg-garden` (DCT Logic, Glitch Aesthetic)
-   **Parent B**: `experiments/chimera-lang` (Genetic VM, Evolution)
-   **Novel Trait**: **Frequency Domain Habitation**. Life that exists only in the compression algorithm.

## Usage

```bash
cargo run --bin jpeg-chimera
```

The simulation starts with a generated noise image. Agents (colored squares) will spawn and begin executing their genetic code, modifying the image in real-time.
