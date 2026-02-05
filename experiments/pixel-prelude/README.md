# Pixel Prelude 🖼️🔐

> "The image is the repository."

**Pixel Prelude** is a steganography tool that embeds binary data into the Least Significant Bits (LSB) of image pixels. It allows you to distribute source code, executables, or secrets inside innocuous looking images.

## Features

*   **Hide**: Embed files into PNG/BMP images.
*   **Reveal**: Extract hidden files from images.
*   **Inspect**: A TUI-based "Spectacles" mode to visualize the hidden LSB noise layer.

## Usage

```bash
# Hide a payload
cargo run -- hide -i cover.png -p secret.txt -o output.png

# Reveal a payload
cargo run -- reveal -i output.png -o recovered_secret.txt

# Inspect the hidden layer (TUI)
cargo run -- inspect -i output.png
```

## How it works

The tool uses a simple LSB substitution algorithm:
1.  The length of the payload (u32, 4 bytes) is encoded first.
2.  The payload bytes are then encoded bit-by-bit into the R, G, B channels of the pixels.
3.  Each pixel can store 3 bits of data.

Capacity = `(Width * Height * 3) / 8` bytes.

## The "Wild" Mode

Run `inspect` to see the image in your terminal. Press `<SPACE>` to toggle the "Hidden Layer" view.
-   **Cover Mode**: Shows the image converted to ASCII grayscale.
-   **Hidden Mode**: visualizes the noise in the lowest bits. If the image is empty, it should be mostly blank. If it has data, it will look like static.

## Dependencies

*   `image`: Pixel manipulation.
*   `ratatui`: Terminal UI.
*   `clap`: CLI argument parsing.

## Warnings

*   **Lossless Only**: Do not save as JPEG! Compression will destroy the hidden data. Use PNG.
