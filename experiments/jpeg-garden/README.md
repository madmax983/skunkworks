# JPEG Garden 🍂🥀

> "Digital rot is not a bug. It is a feature of the universe."

**JPEG Garden** is a simulation of "bit rot" and "generational loss" applied to images.
It uses a custom implementation of the Discrete Cosine Transform (DCT) to decompose an image into frequency coefficients, then simulates entropy by:
1.  **Quantization Rot**: Zeroing out high-frequency coefficients (detail loss).
2.  **Noise Moss**: Adding random noise to coefficients (fungal growth).
3.  **Chromatic Drift**: Shifting the DC components of color channels (color fading).

It then reconstructs the image using Inverse DCT (IDCT) in real-time.

## Resurrection ⚰️ -> 🌱

This project was originally condemned to the `graveyard/` due to lack of documentation.
It has been **Resurrected** by Genesis (The Archivist) as a study in digital preservation and decay visualization.

## Features

-   **Real-time Decay**: Watch the image rot before your eyes.
-   **Sonification**: The "scream" of the dying image is procedurally generated based on the total error metric.
    -   Low error: A low hum.
    -   High error: A distorted screech.
-   **Heatmap View**: Press `V` to invert/highlight the decay.
-   **Reset**: Press `R` to generate a new pristine image.

## Usage

```bash
cargo run --bin jpeg-garden
```

**Controls:**
-   `Space`: Pause/Resume decay.
-   `R`: Reset to a fresh image.
-   `V`: Toggle View Mode (Normal / Heatmap).
-   `Esc`: Quit.

## Technical Details

-   **Stack**: `macroquad`, `image`, `rand`.
-   **Core Logic**: `src/garden.rs` manages the DCT/IDCT and entropy rules.
-   **Audio**: Generates WAV data in memory and plays it via `macroquad::audio`.
