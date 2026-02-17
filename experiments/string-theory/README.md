# String Theory 🎻

A physical modeling synthesizer where the codebase is the instrument.

## Concept

Each file in the repository is represented as a vibrating string.
- **Length/Pitch**: Determined by file size (smaller = higher pitch).
- **Timbre/Decay**: Determined by file extension (e.g., `.rs` sounds metallic, `.md` sounds dull).
- **Pluck**: Interaction via mouse movement.

## Controls

- **Mouse Movement**: Pluck strings. Faster movement = louder sound.
- **Scroll Wheel / Arrow Keys**: Navigate horizontally through the file system "harp".
- **ESC**: Exit.

## Technical Details

- **Audio**: Custom Karplus-Strong string synthesis engine using `cpal`.
- **Visuals**: `macroquad` with simple spring physics for string vibration.
- **Physics**: Mass-spring-damper model for visual feedback.

## Usage

```bash
# Run in silent visualization mode (default)
cargo run --release --bin string-theory [path/to/scan]

# Run with audio enabled (requires ALSA/JACK on Linux)
cargo run --release --bin string-theory --features audio [path/to/scan]
```

If no path is provided, it scans the current directory.
