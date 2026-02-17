# Neuro Calligraphy 🧬✒️

**"The Word made Flesh."**

A hybrid experiment combining **Resonant Glyphs** (Font Parsing & Geometry) with **Spinal Rhythms** (CPG Neural Networks & Physics).

Each character is a living organism. Its outline is a closed loop of muscles driven by a Central Pattern Generator (CPG) neural network. The neural impulses travel around the perimeter of the glyph, causing it to writhe and pulsate in an organic rhythm.

## Lineage
- **Parent A**: `experiments/resonant-glyphs` - Provided the ability to parse font outlines into geometry.
- **Parent B**: `experiments/spinal-rhythms` - Provided the Izhikevich Neural Network (CPG) and Verlet Physics engine.
- **Novel Trait**: **Living Typography**. The static vector data of a font becomes the skeleton for a soft-body simulation.

## Controls
- **Click**: Disturb the fluid/physics (push the letters).
- **Space**: Excite the neural networks (increase drive).

## Implementation Details
- **Font Parsing**: `ttf-parser` extracts the glyph contours.
- **Resampling**: Contours are resampled to equidistant points to ensure stable physics simulation.
- **Creature Construction**: A "Ribbon" morphology is created by extruding points along the normal of the contour.
- **Neural Control**: A ring of Izhikevich neurons (one per segment) drives the contraction of longitudinal muscles, creating a peristaltic wave.
