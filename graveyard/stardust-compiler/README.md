# Stardust Compiler ⚛️

> "The universe is full of magical things patiently waiting for our wits to grow sharper." - Eden Phillpotts

**Stardust Compiler** is a steganography tool that hides executable code (or any data) within the glittering noise of a procedurally generated nebula. It doesn't just hide the data; it visualizes the extraction process as a particle beam scanning the stars.

## 🌌 Concept

Data is noise. Noise is data.
By embedding information into the Least Significant Bits (LSB) of a chaotic image, we can hide entire programs in plain sight.
To the naked eye, it's a picture of space.
To the **Stardust Compiler**, it's a library of forbidden knowledge.

## 🚀 Features

- **Procedural Cover Generation**: Don't have a cover image? We'll forge a unique nebula for you using `macroquad` and `rand`.
- **Visual Encoding/Decoding**: Watch as the data is injected into the stars or extracted from the void.
- **Compression**: Uses `flate2` (zlib) to maximize the payload capacity of your star charts.
- **CLI + GUI Hybrid**: Run it from the terminal, watch it on the screen.

## 🛠️ Usage

### Encode
Hide a file inside a nebula:

```bash
cargo run --release -- encode secrets.txt --output nebula.png
```

Or use your own cover image:

```bash
cargo run --release -- encode payload.wasm --cover telescope.png --output dark_matter.png
```

### Decode
Extract the hidden data:

```bash
cargo run --release -- decode nebula.png --output secrets_recovered.txt
```

If the data is text, you can omit `--output` to print it to the console (and see it on screen).

## 🧪 Experiments

This project is part of the **Genesis** moonshot collection.
It combines:
- **LSB Steganography**
- **Code Distribution via Images**
- **Procedural Art**

## ⚠️ Warning

The visualizer requires a display. If running on a headless server, it might panic (but the data will still be safe in the void... probably).

---
*Genesis: The best encryption is invisible.*
