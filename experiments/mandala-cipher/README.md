# Mandala Cipher ☸️

> "The universe is built on a plan the profound symmetry of which is somehow present in the inner structure of our intellect." - Paul Valéry

**Mandala Cipher** is a visual steganography tool that encodes information into sacred geometry. It transforms text into a radially symmetric mandala, where the placement, color, and shape of "Jewels" carry the data.

## 🌟 Concept

The cipher uses a **Canonical Sector** (a slice of the pie) to store the payload. This sector is then replicated `N` times to form a complete mandala.
- **Data Encoding**: Each byte is split into two 4-bit nibbles.
- **Jewels**: Each nibble determines a Jewel's attributes:
  - **Shape** (2 bits): Circle or Square.
  - **Color** (3 bits): Red, Green, Blue, Yellow, Purple, Cyan, White, Black.
- **Chaff**: Unused slots are filled with "Chaff" jewels (Triangles and Diamonds), which are visually consistent but ignored by the decoder.

## 🎮 Usage

Run the interactive visualizer:

```bash
cargo run -p mandala-cipher
```

- **Type**: Enter text to encode it in real-time.
- **Backspace**: Delete characters.
- **S**: Save the current mandala as `mandala_output.png`.

## 🧠 Decoding

The `decode` function in `lib.rs` reverses the process. It ignores Triangles and Diamonds (noise) and reconstructs the byte stream from Circles and Squares.

## 🧪 Status

- [x] Core Logic (Encode/Decode)
- [x] Visual Renderer (Macroquad)
- [x] Rotation & Bloom
- [ ] Computer Vision Decoder (Recover from PNG)

## 🌌 Moonshot

This experiment combines **Visual Ciphers** with **Source Code Obfuscation** principles. The goal is to distribute code as art.
