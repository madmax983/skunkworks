# Codex Void ⚛️🔐

**"The stars are aligned. The void is speaking."**

## Concept
Codex Void is a steganographic system that encodes data into procedural star maps. It is a **Visual Cipher** where the "Art" (a night sky) is the "Message".
Instead of hiding bits in the noise (LSB), the bits *are* the stars.
Each star is a glyph. Its radial corona (rays) encodes a byte.

## Moonshot
**Visual Ciphers + QR Code Alternatives**
- **Visuals:** A generative star field with salt-noise background stars and nebula effects (implied by void color).
- **Cipher:** Radial Binary Encoding. 8 rays = 8 bits.
- **Robustness:** Scanner uses brightness thresholding and spatial sorting to recover the data stream from the image.

## Usage

### Build
```bash
cargo build --release -p codex-void
```

### Scribe (Encoder)
Encode text into a star map:
```bash
cargo run -p codex-void --bin scribe -- "Hello Cosmos" -o cosmos.png
```

### Observatory (Decoder/Visualizer)
Launch the visual scanner:
```bash
cargo run -p codex-void --bin observatory -- cosmos.png
```
- **Hover** over a star to see its value.
- **Green Reticle**: Detected star.
- **Red Rays**: Detected bits (1s).
- **Decoded Signal**: Appears at the bottom.

## Architecture
- **Glyph**: A star core with 8 potential rays at 45-degree intervals.
- **StarMap**: A grid of glyphs with jitter and salt noise (background stars).
- **Scanner**: A computer vision module that thresholds the image, finds star centroids, and samples the ray positions to decode bytes.

---
*Built by Genesis ⚛️*
