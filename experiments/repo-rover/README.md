# Repo Rover 🚜

**Explore your codebase like an alien planet.**

Repo Rover is a TUI-based exploration game where you drive a rover across the terrain of your file system. Directories are distinct sectors, and files are artifacts to be scanned.

## Features

- **Physics-based Movement:** Thrust, rotate, and drift through the void of your repository.
- **Spiral Layout:** Directory contents are distributed in a Fermat's Spiral pattern.
- **Interaction:**
  - `Arrow Keys` / `WASD`: Drive.
  - `Space`: Scan nearby files (view metadata).
  - `Enter`: Dive into nearby directories (load new sector).
  - `+` / `-`: Zoom in/out.

## Controls

| Key | Action |
| --- | --- |
| `↑` / `w` | Thrust Forward |
| `↓` / `s` | Thrust Backward |
| `←` / `a` | Rotate Left |
| `→` / `d` | Rotate Right |
| `Space` | Scan File |
| `Enter` | Enter Directory |
| `+` / `-` | Zoom |
| `q` / `Esc` | Quit |

## Technical Details

- **Engine:** `ratatui` + `crossterm`.
- **Physics:** Simple 2D vector physics with friction.
- **World Generation:** `std::fs` scanning mapped to 2D coordinates.

## Future Ideas

- Terrain generation based on file complexity/lines of code.
- "Mining" files to refactor them.
- Multiplayer?
