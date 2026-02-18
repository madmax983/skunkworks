# Circuit Sigil ⚛️

**"The Commit is the Circuit."**

## Concept
Circuit Sigil is a "Moonshot" experiment that combines **Image Authentication** with **Procedural Art**.
It generates a unique, deterministic Printed Circuit Board (PCB) design from any text string (like a Git commit hash).
Hidden within the copper pads of the circuit is the author's name and commit message, encoded via LSB steganography.

This creates a system where the "Visual Hash" of a commit is not just a random avatar, but a complex, verifiable artifact that contains the commit metadata itself.

## Moonshot Combination
*   **Image Authentication + Git Commit Verification**: The image visually represents the commit hash, and steganographically proves the commit content.
*   **Pattern Encoding**: The circuit layout is a 2D barcode of the hash.

## Usage

### Generate a Sigil
```bash
cargo run --release -- generate "commit-hash-123" -a "Genesis" -m "Initial Commit" -o sigil.png
```
This creates `sigil.png`. The layout is derived from `"commit-hash-123"`. The Author and Message are hidden in the gold pads.

### Verify a Sigil
```bash
cargo run --release -- verify sigil.png "commit-hash-123"
```
This:
1.  Regenerates the expected circuit layout from the hash.
2.  Compares it pixel-by-pixel with `sigil.png` (ignoring LSBs) to ensure visual integrity.
3.  Extracts and prints the hidden message from the pads.

## Technical Details
*   **Generator**: Uses `ChaCha20` seeded with `SHA-256` of the input to place pads and route traces using a Manhattan routing algorithm.
*   **Steganography**: Embeds data into the LSB of the RGB channels of the pixels within the "Gold Pads". The pad locations are deterministic based on the hash, acting as a symmetric key.
*   **Stack**: Rust, `image`, `rand`, `sha2`. No external crypto libraries used for encryption (only hashing).

---
*Built by Genesis ⚛️*
