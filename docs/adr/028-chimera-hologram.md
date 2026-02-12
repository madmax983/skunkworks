# 28. Chimera Holographic Memory

Date: 2024-05-22

## Status

Proposed

## Context

The Chimera VM's primary memory structures (`grid` and `stack`) store discrete values (`Int`, `String`, `Junction`). This discrete storage is brittle: a single bitflip or accidental overwrite can completely destroy a program or data structure.

Biological memory systems, however, are believed to be distributed and resilient. Karl Pribram's "Holonomic Brain Theory" suggests that memory is stored as interference patterns rather than in specific locations. In such a system, damage to a part of the storage medium degrades the resolution of the memory but does not destroy it entirely.

To advance the "Nova" biological simulation features, Chimera needs a memory substrate that exhibits these properties:
1.  **Distributed:** Information is spread across the entire medium.
2.  **Resilient:** Local damage only adds noise, not catastrophic failure.
3.  **Associative:** Retrieval is based on content/frequency resonance, not just address.

## Decision

We will implement a **Holographic Memory System** within the Chimera VM, utilizing a complex-valued 2D grid (`hologram_grid`) and a set of OpCodes to interact with it via Fourier Transforms.

### 1. Data Structure
The `hologram_grid` will be a `Vec<Vec<(f64, f64)>>` of size 16x16 (matching the main `grid`), storing Complex numbers (Real, Imaginary).

### 2. Encoding (Interference)
The `Interfere` OpCode will encode a DNA strand into the grid by treating genes as frequencies.
-   **Gene Index** maps to Frequency $(u, v)$.
-   **OpCode** maps to Phase $(\phi)$.
-   **Argument** maps to Amplitude $(A)$.
-   The system performs an Inverse DFT to accumulate the wave pattern onto the grid.

### 3. Decoding (Refraction)
The `Refract` OpCode will reconstruct DNA from the grid.
-   It performs a Forward DFT to extract frequency components.
-   Strong signals (high magnitude) are decoded back into OpCodes and Arguments based on phase and amplitude.
-   Weak signals are discarded as noise.

### 4. Quantum Cybernetics
To bridge the gap between "wave" (hologram) and "particle" (grid) representations, we introduce "Quantum" OpCodes:
-   `QuantumScribe`: Collapses the wave function (Phase) at a specific point into an ASCII character on the main grid.
-   `QuantumScan`: Encodes a character from the main grid into the hologram as a wave source.

## Consequences

### Positive
-   **Fuzzy Storage:** Genetic information can be stored and retrieved even if the grid is partially "damaged" (e.g., by `PhaseMutate` or decay).
-   **Data Density:** Multiple strands can be superimposed on the same grid, similar to how multiple holograms can be stored on the same film at different angles.
-   **Biological Realism:** Mimics the distributed nature of neural memory.

### Negative
-   **Computational Cost:** The current implementation uses a naive DFT ($O(N^4)$ for 2D grid update per gene), which is computationally expensive compared to direct array access. However, for a small 16x16 grid, this is acceptable.
-   **Complexity:** Debugging interference patterns is non-intuitive compared to inspecting a stack.
-   **Precision Issues:** Repeated encoding/decoding may introduce floating-point errors, causing "mutation" of the stored DNA.

### Neutral
-   **Dependency:** We implement the DFT manually to avoid heavy dependencies like `rustfft` for such a small grid size, keeping the build lightweight.
