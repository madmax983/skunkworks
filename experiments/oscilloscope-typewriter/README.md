# Oscilloscope Typewriter

A "Moonshot" experiment combining **Bezier Manipulation** and **Audio Visualization**.

This experiment treats text as a signal. Each character you type adds a "voice" (frequency/waveform) to the mix. The glyphs themselves are rendered not as static pixels, but as vector paths traced by a simulated electron beam. The beam is modulated by the very frequency the character represents, causing the letters to vibrate and distort in a standing wave pattern.

## Concept
"Sonic Calligraphy": What if you could see the sound of the letters you type?
The outlines of the glyphs are extracted using `rusttype`, flattened into points, and then displaced along their normal vectors based on a synthesized waveform.

## Controls
- **Type**: Add characters and voices.
- **Backspace**: Remove characters and voices.
- **Visuals**: The green "phosphor" glow is achieved by multi-pass rendering with varying width and opacity.

## Tech Stack
- `macroquad`: 2D/3D rendering and windowing.
- `rusttype`: Font parsing and glyph outline extraction.
