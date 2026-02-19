# Mayan Git 🏺

> "The repository history is a Long Count."

A TUI visualization of your Git history, using the **Mayan Long Count Calendar** instead of Gregorian dates.

## Concept
Every commit is a sacred event in time. We map the Unix timestamp to the Mayan Long Count (Baktun.Katun.Tun.Uinal.Kin).
The epoch (0.0.0.0.0) is August 11, 3114 BCE.

## Features
- **Long Count Converter**: Converts Git commit timestamps to Mayan dates.
- **Stela Visualization**: Renders the date as a vertical stack of Mayan glyphs (ASCII art).
- **Interactive History**: Scroll through your repository's timeline.

## Usage
Run from within any git repository (or this one):
```bash
cargo run --release
```

## Controls
- `Up/Down` or `k/j`: Navigate history
- `q` or `Esc`: Quit

## Lineage
- **Core**: `git-associates` for repository access.
- **UI**: `ratatui` for the terminal interface.
- **Math**: `chrono` for time manipulation.
