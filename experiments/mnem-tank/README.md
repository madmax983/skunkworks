# Acoustic Memory Decay (mnem-tank) 🗄️🌊

> "The data is not gone. It was shaken apart by the noise."

**Mnemonic Tank** is a hybrid experiment combining the codebase decay visualization of `mnem-rot` with the 2D acoustic wave simulation of `ripple-tank`.

## 🧬 Lineage

- **Parent A:** `experiments/mnem-rot`
  - *Alleles Inherited:* Codebase graph parsing, file health entropy, stochastic text corruption (glitching), and camera controls.
- **Parent B:** `experiments/ripple-tank`
  - *Alleles Inherited:* 2D Finite Difference Time Domain (FDTD) acoustic wave simulation, physical pressure waves, and `resonance-audio`.

## 🔬 Phenotype

**Acoustic Memory Decay:** Instead of nodes slowly decaying over time uniformly, they exist within an acoustic wave tank. High-amplitude pressure waves physically damage the nodes, reducing their health. As their health degrades, the node's code corrupts and glitches. Damaged nodes act as acoustic emitters, adding high-frequency noise back into the wave tank, creating a cascading failure of data corruption driven by acoustic resonance.

## 🎮 Controls

- **Left Click:** Pluck the wave tank (send out a pressure wave that damages nodes).
- **Right Click + Drag:** Pan the camera.
- **Scroll:** Zoom in/out.
- **Hover:** Heal a node and view its (potentially corrupted) content.

## 📦 Usage

Run from the root of a workspace:

```bash
cargo run -p mnem-tank
```

To hear the data corruption (requires ALSA on Linux):

```bash
cargo run -p mnem-tank --features audio
```
