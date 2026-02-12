# 🧬 Chimera Hologram

**"The code you see depends on the angle you look from."**

`chimera-hologram` is a hybrid experiment combining the **Holographic Memory** concepts from `holographic-brain` with the **Genetic Virtual Machine** of `chimera-lang`.

## 🧬 Concept: Subjective Code Execution

In traditional computing, code is static. Bytes on disk are immutable.
In `chimera-hologram`, code is a **Holographic Interference Pattern** (stored in the Frequency Domain).

To execute the code, the VM must "shine a reference beam" through the hologram. The angle of this beam (Phase Shift) determines the interference pattern that emerges in the Spatial Domain (Real Space).

This means the **same** DNA object can contain infinite variations of programs, accessible only by changing your perspective (Phase Shift).

## 🔬 Lineage

- **Parent A**: `experiments/holographic-brain`
  - *Traits Inherited*: FFT/IFFT memory storage, Frequency Domain manipulation, Complex number coefficients.
- **Parent B**: `experiments/chimera-lang`
  - *Traits Inherited*: The Genetic Virtual Machine, OpCodes, execution semantics.

## 🧪 Novel Traits

- **Holographic DNA**: The genome is a vector of Complex numbers, not OpCodes.
- **Phase-Dependent Phenotype**: Rotating the phase of the coefficients fundamentally alters the decoded instructions.
- **Spectrum Visualization**: The TUI visualizes the raw frequency spectrum alongside the decoded phenotype.

## 🎮 Controls

- **Left/Right Arrows**: Adjust the Phase Shift (Reference Beam Angle). Watch the code rewrite itself in real-time!
- **R**: Generate a new random Holographic Genome.
- **Q**: Quit.

## 🏗️ Architecture

1.  **Genome**: `Vec<Complex<f64>>` (Frequency Domain).
2.  **Decode Step**:
    - Apply Phase Shift: $C' = C \cdot e^{i\theta}$
    - Inverse FFT: $Signal = \mathcal{F}^{-1}(C')$
    - Mapping: $OpCode = |Re(Signal)| \pmod N$
3.  **Execution**: The decoded OpCodes are fed into a fresh `ChimeraVM` instance.

## 📜 License

MIT
