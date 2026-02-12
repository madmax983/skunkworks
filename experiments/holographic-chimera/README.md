# 🧬 Holographic Chimera

**Where the genome is an interference pattern.**

## Concept

In traditional genetic algorithms (and `chimera-lang`), the genome is a linear sequence of instructions carried by the organism.

In **Holographic Chimera**, the genome is encoded into the environment itself as a **Hologram** (frequency domain representation).

-   **The World** is a sea of overlapping interference patterns.
-   **Organisms** do not carry DNA. Instead, they act as "Tuners".
-   Each organism has a specific **Reference Angle** (frequency shift).
-   To "read" their genetic instructions, they perform an Inverse FFT on the local holographic field using their reference angle.
-   If they tune into a valid frequency, coherent instructions (OpCodes) appear in the reconstructed image.
-   If they are out of tune, they see noise.

## Lineage

-   **Parent A**: `experiments/chimera-lang` (The Splice Surgeon)
    -   *Inherited*: The concept of OpCodes (`Consume`, `Photosynthesize`, `Divide`, `Move`) and the metabolic loop.
-   **Parent B**: `experiments/hologram-text` (Nova)
    -   *Inherited*: The physics of storing information in the frequency domain, FFT/IFFT reconstruction, and the "Angle of View" mechanic.

## Mechanism

1.  **Seeding**: The world is seeded with 4 "Channels" of instructions:
    -   Channel 0 (Angle 0,0): `Consume` (Food)
    -   Channel 1 (Angle 20,10): `Photosynthesize` (Sun)
    -   Channel 2 (Angle -20,-10): `Divide` (Reproduction)
    -   Channel 3 (Angle 10,-20): `Move` (Migration)

2.  **Simulation**:
    -   Organisms move around the grid.
    -   Each tick, they scan the hologram at their location using their channel index.
    -   The scanned image is compared to known "Ideal" bitmaps of OpCodes (Optical Code Recognition).
    -   If a match is found, the organism executes the instruction.

3.  **Evolution**:
    -   When organisms divide, their offspring inherit their position but might mutate their **Channel Index**.
    -   This allows the population to switch strategies (e.g., from Eating to Photosynthesizing) by "tuning" into a different frequency of reality.

## Running

```bash
cargo run --release
```

**Controls:**
-   `Q`: Quit

## Visualization

-   **Background (Green)**: Intensity of the "Food" channel (Hologram Magnitude).
-   **Organisms**:
    -   `C` (Green): Eaters
    -   `P` (Yellow): Photosynthesizers
    -   `D` (Cyan): Dividers
    -   `M` (Magenta): Movers

---
*Splice Surgeon 🧬*
