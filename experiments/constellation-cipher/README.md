# 🌌 Constellation Cipher

> "The best encryption is invisible. Steganography doesn't just protect data—it denies its existence." - Genesis

**Constellation Cipher** is a Moonshot tool that hides information in plain sight as **Star Maps**.
It encodes data (text, source code, binaries) into the positions and spectral properties of stars in a generated PNG image.
The result looks like a night sky or a deep field astronomy photograph.

## 🔮 Concept

-   **Cover**: A black void (space).
-   **Payload**: Your secret data (e.g., source code).
-   **Cipher**:
    -   **Position**: Determined by a CSPRNG (`ChaCha20`) seeded with your secret key. The stars appear at random locations, but their positions are deterministically reproducible by the recipient.
    -   **Magnitude/Color**: The byte value of the payload is encoded in the star's color (Red Channel/Hue).
    -   **Steganography**: The map is filled with 99% empty space. The stars are sparse. Decoys (noise stars) fill the void to mask the data density.

## 🛠️ Usage

### CLI

The tool provides a CLI for encoding and decoding.

```bash
# Build the CLI
cargo build --bin cli

# Encode a file
./target/debug/cli encode --input secret.txt --output starmap.png --key "my-secret-password" --width 1024 --height 1024

# Decode a file
./target/debug/cli decode --input starmap.png --output recovered.txt --key "my-secret-password"
```

### Interactive Viewer

Explore the hidden data visually.

```bash
# Run the Viewer
cargo run --bin viewer -- starmap.png "my-secret-password"
```

-   **Pan**: Arrow Keys
-   **Zoom**: Mouse Wheel
-   **Decode**: Hover over a star (Green Circle) to reveal its hidden byte/character.

## 🧪 Experiments

This project explores:
-   **Visual Ciphers**: Encoding data in aesthetic patterns.
-   **Steganography**: Hiding signals in noise (random star distribution).
-   **Code Distribution**: distributing a program as an image of the universe it simulates.

## ⚠️ Warning

This is a **Moonshot** experiment. It is not cryptographically audited.
Do not use for high-stakes secrets.
The "Encryption" relies on `ChaCha20` stream generation, which is robust, but the implementation focuses on "Hiding" (Steganography) rather than "Hardening".
