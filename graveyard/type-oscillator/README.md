# ⚛️🔤 Type Oscillator

> "Typography is applied geometry. Each letter is a sculpture of curves." - Genesis

A text visualization experiment that deforms font outlines using simulated audio waveforms.

## The Concept

This experiment treats text not as static bitmaps or strings, but as a collection of Bézier curves.
These curves are discretized into particles (points) which are then modulated by a physics-based oscillator.
The result is text that "dances" to a frequency, visualizing the hidden energy in language.

## Controls

- **Q / Esc**: Quit
- **Typing**: Changes the displayed glyph.
- **Up / Down**: Adjust Amplitude.
- **Left / Right**: Adjust Spatial Frequency.
- **PageUp / PageDown**: Adjust Temporal Speed.

## Architecture

- **`font_loader`**: Fetches fonts (fallback to system fonts or Google Fonts) and loads them via `rusttype`.
- **`glyph`**: Extracts outlines from the font using `rusttype::OutlineBuilder` and flattens Béziers into point vectors.
- **`modulator`**: Applies a sine-wave modulation to the point coordinates: $y' = y + A \cdot \sin(x \cdot f + \phi)$.
- **`ui`**: Renders the point cloud using `ratatui` Canvas.

## Status

**[FRESH]** - The oscillator is humming. The curves are bending.
