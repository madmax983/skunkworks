# Chaos Hologram 🌀🔮

> "The chaotic pendulum traces the spectral geometry of a text's ghost potential."

**Chaos Hologram** is a TUI hybrid experiment combining the chaotic double-pendulum attractor of `chaos-flock` with the spectral FFT-based potential field reconstruction of `hologram-text`.

## 🧬 Lineage

- **Parent A:** `chaos-flock`
  - *Alleles Inherited:* `DoublePendulum` dynamics and `Vec2` chaotic integration tracking the complex geometry of a multi-jointed physical system.
- **Parent B:** `hologram-text`
  - *Alleles Inherited:* `Hologram` text grid generation, phase-modulated 2D Fast Fourier Transform (FFT) spectrum mapping, and shifting inverse-FFT reconstruction.

## 🔬 Phenotype

**Spectral Chaos:** In `hologram-text`, the structure reconstructs cleanly when viewing angles (-20, -10) precisely match recording angles. In `chaos-hologram`, the viewing angles are violently coupled to the tip position of a chaotic double pendulum.

As the pendulum swings unpredictably, it pulls the observer in and out of phase with the object wave. Order and structure (the words) break down into abstract noise and violently reform into focus dynamically.

## 🎮 Controls

- **Type alphanumeric keys** to change the holographic text structure dynamically.
- **Backspace** to delete characters from the hologram.
- **`K` / `k`** to kick the pendulum and induce more chaos.
- **`Esc`** to quit the simulation.

## 📦 Usage

Run from the root of a workspace:

```bash
cargo run -p chaos-hologram
```
