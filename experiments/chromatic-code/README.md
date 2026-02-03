# Chromatic Code 🌈🔐

> "The best encryption is invisible. Steganography doesn't just protect data—it denies its existence."

**Chromatic Code** is a steganography tool that hides source code (or any text) inside procedurally generated "Plasma" art. The image acts as a visual cipher: to the naked eye, it's just a psychedelic gradient. To the decoder, it's a carrier signal.

## Features

-   **Procedural Cover Generation**: Creates unique "Plasma" images on the fly. No need for an existing image to hide things in.
-   **LSB Steganography**: Embeds data into the Least Significant Bits of the RGB channels.
-   **TUI Visualization**: A "Matrix-style" viewer that renders the image in the terminal (using half-blocks) and animates the decoding process in real-time.
-   **CLI Support**: Encode, Decode, and View commands.

## Usage

### Demo
Run the self-contained demo to see it in action (hides its own source code in a generated image):
```bash
cargo run -p chromatic-code -- demo
```

### Encode
Hide a text file inside a new image:
```bash
cargo run -p chromatic-code -- encode -i src/main.rs -o secret.png --width 400 --height 200
```

### View (TUI)
View and decode an image in the terminal:
```bash
cargo run -p chromatic-code -- view -i secret.png
```

### Decode (CLI)
Extract the hidden text to a file:
```bash
cargo run -p chromatic-code -- decode -i secret.png -o extracted_code.rs
```

## How it Works

1.  **Generation**: We generate a continuous plasma field using summed sine waves.
2.  **Embedding**: We flatten the text into bits. We iterate through the image pixels and replace the LSB of each Red, Green, and Blue channel with our data bits.
3.  **Visualization**: The TUI uses `ratatui` to render the image using `▀` (HalfBlock) characters, effectively giving us two pixels per character cell. The "decoding" animation is purely visual flair.

## Constraints

-   Capacity is limited by image resolution (`Width * Height * 3` bits).
-   Requires `ratatui` compatible terminal.

---
*Built by Genesis ⚛️*
