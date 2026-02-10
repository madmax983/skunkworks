# Pixel Archivist 🌌

> "The Repository is the Image."

**Pixel Archivist** is a steganographic tool that archives entire directories into procedurally generated "Nebula" images.

## Features

- **Pack:** Compress a directory into a `.tar.gz` and embed it into the LSB (Least Significant Bits) of a generated image.
- **Unpack:** Extract the payload from the image and restore the directory.
- **Procedural Art:** The cover image is not random noise; it is a deterministic "Nebula" generated from the hash of the payload. The image *is* the fingerprint of the data.
- **Visualization:** Includes a `macroquad` viewer to inspect the archive (WIP).

## Usage

```bash
# Pack a directory
cargo run -- pack ./my-source-code output.png

# Unpack
cargo run -- unpack output.png ./restored-source-code

# View
cargo run -- gui output.png
```

## How it works

1. **Compression:** The directory is tarred and gzipped.
2. **Hashing:** The compressed data is hashed (SHA-256).
3. **Generation:** The hash seeds a PRNG which generates a cosmic nebula image. The image size is automatically calculated to fit the data.
4. **Embedding:** The data is embedded into the image using 2-bit LSB steganography (6 bits per pixel).

## Moonshot
This tool explores the concept of **Visual Storage**. What if your backups looked like art? What if you could hang your source code on the wall?
