# Code Concerto 🎻

**Code Concerto** is a cross-sensory translator that turns Rust source code into generative music and visualizes the playback in a TUI.

## Concept
The project maps the Abstract Syntax Tree (AST) of a Rust program to musical structures:
- **Structs** become **Major Chords**.
- **Enums** become **Minor Chords**.
- **Functions** become **Melodies**.
- **Identifiers** determine the pitch/root note via hashing.

## Usage

```bash
cargo run -p code-concerto -- [path_to_rust_file]
```

If no file is provided, it plays its own source code (`src/main.rs`).

## Output
1. **concerto.wav**: A WAV file containing the generated audio.
2. **TUI**: A real-time visualization of the "playback", highlighting the code structure being played.

## Technical Details
- **Parser**: `syn`
- **Synthesis**: `hound` (Pure Rust WAV generation)
- **Visualization**: `ratatui`
- **Mapping**: Hashed identifiers to frequency spectrum.

## Notes
Due to sandbox limitations (missing ALSA), real-time audio playback is disabled. The system generates a WAV file for external listening while simulating the playback visually.
