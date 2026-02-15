# Hanging Gardens of Entropy

> "The code grows, the rain falls, the memory fades."

A hybrid experiment combining `babylonian-garden` and `entropic-rain`.

## Lineage
- **Parent A:** `experiments/babylonian-garden` (Procedural L-Systems, Sexagesimal DNA)
- **Parent B:** `experiments/entropic-rain` (Code Erosion, Particle Physics)
- **Concept:** Git additions create procedurally generated "hanging gardens" (L-System plants) that grow from the top of the terminal. Git deletions create "entropic rain" that falls and erodes (dissolves) the plants upon contact.
- **Novel Trait:** **Ecosystem Balance**. Can the codebase grow faster than it decays? The visualization shows the struggle between feature development (Growth) and technical debt/refactoring (Entropy).

## Controls
- `SPACE`: Pause/Resume
- `+ / -`: Increase/Decrease playback speed
- `q`: Quit

## Implementation
- **TUI:** `ratatui` + `tui-shared`
- **Data:** `git-associates` for commit history
- **Simulation:** Custom particle physics + L-System collision detection
