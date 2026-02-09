# L-System Garden 🌿

A terminal-based explorer for Lindenmayer Systems (L-Systems), allowing you to visualize fractals and plant-like structures using simple string rewriting rules.

## Controls

| Key | Action |
| --- | --- |
| `Tab` | Cycle through presets (Dragon Curve, Sierpinski, etc.) |
| `Space` | Increase iterations (Growth) |
| `r` | Reset iterations to 0 |
| `+` / `-` | Zoom In / Out |
| `Arrows` | Pan the view |
| `q` / `Esc` | Quit |

## How it works

An L-System consists of:
- **Axiom**: The starting string (e.g., "F").
- **Rules**: How to rewrite characters (e.g., "F" -> "F+F-F").
- **Turtle Interpretation**:
  - `F`, `G`: Move forward and draw a line.
  - `+`, `-`: Turn left/right by a specific angle.
  - `[`: Push current state (position, angle) to stack.
  - `]`: Pop state from stack (return to previous position).

## Presets Included
- Dragon Curve
- Sierpinski Triangle
- Fractal Plant
- Koch Curve
- Gosper Curve
