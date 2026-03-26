# Holographic Strings (hologram-strings)

A TUI visualization of spectral acoustics, crossing the structural strings of `ferrous-strings` with the FFT spectral visualization of `hologram-text`.

## Lineage

* **Parent A:** `experiments/ferrous-strings` - Provides the underlying concept of physical vibrating strings.
* **Parent B:** `experiments/hologram-text` - Provides the FFT-based transformation that converts 2D spatial data into a holographic spectral projection.

## Novel Trait

**Spectral Acoustics.** We observe the discrete acoustic strings entirely through their frequency domain interference pattern via FFT. As the strings vibrate and resonate with random forces, their continuous spatial waves act as an optical interference pattern. We are effectively viewing the *acoustic resonance* of multiple kinetic strings through a holographic lens.

## Running

```bash
cargo run -p hologram-strings
```

* **Controls:**
  * `Arrows`: Shift the reconstruction angle.
  * `Enter`: Reset angle to recording angle.
  * `Esc`: Quit.
