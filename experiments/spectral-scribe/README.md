# Spectral Scribe ⚛️

**Spectral Scribe** is a steganography tool that hides messages inside audio spectrograms.
It combines "Spectral hiding" with "Code distribution".

## Concept
The tool converts text into a bitmap, maps the bitmap to frequency bands, and generates an audio file (`output.wav`).
When played back and viewed in a spectrogram (like Audacity or the built-in visualizer), the hidden message appears.

## Usage

### Run
```bash
cargo run --release
```

### Controls
*   **Write Mode**: Type your message. Press `ENTER` to generate `output.wav` and play it.
*   **Read Mode**: (Automatic after generating) Visualizes the spectrogram of the generated file.
*   **ESC**: Return to Editor.

## Implementation
*   **Encoder**: Maps 8-row bitmap fonts to frequency bins. Uses Inverse FFT (`rustfft`) to generate time-domain audio.
*   **Decoder**: Performs Forward FFT on audio chunks and thresholds the magnitude to recover the bitmap. Uses `font8x8` for text recovery.
*   **Visualizer**: Real-time scrolling spectrogram using `macroquad`.

## Moonshot Status
*   [x] Spectral Hiding
*   [x] Code/Text Distribution via Audio
*   [x] Rust Performance (FFT)
*   [x] Visuals (Macroquad)
