
## [Biomorph Flow]
**Concept:** A Gray-Scott reaction-diffusion simulation visualized in TUI using `ratatui` Canvas. It supports interactive "seeding" with the mouse and tweaking of Feed/Kill rates to explore pattern space (Coral, Mitosis, Loops).
**Fate:** Merged
**Lesson:** TUI `Canvas` widgets are surprisingly capable of rendering dense 2D fields if you optimize by batching points by color instead of drawing individual pixels. Reaction-diffusion is computationally cheap enough for the main thread.

## [Semantic Spy]
**Concept:** A TUI inspector for `tui-semantic` snapshots. It reads a JSON snapshot from stdin and provides a 3-pane layout: Entity List, Canvas Visualizer, and JSON Details.
**Fate:** Merged
**Lesson:** Simple tools that bridge the gap between "Raw JSON" and "Visual Understanding" are incredibly valuable for debugging. `ratatui` makes building these inspection tools trivial.

## [Diff Drift]
**Concept:** A racing game where the track is generated from `git log` history. Commit hashes determine curvature, message lengths determine straightaways.
**Fate:** Merged
**Lesson:** `ratatui`'s `Canvas` widget is powerful enough for simple pseudo-3D or top-down scrolling games. Visualizing git history as a physical space creates a unique connection to the code.

## [Semantic DVR]
**Concept:** A "Time Machine" for TUI apps. It records a stream of semantic snapshots (JSONL) and provides a TUI player to replay, pause, seek, and inspect the history. Includes a 'Bouncing Ball' producer as a demo.
**Fate:** Merged
**Lesson:** Splitting the "Simulation" (Producer) from the "Visualization" (Player) via a standard protocol (JSON stream) allows for powerful tooling like rewind/replay without complicating the simulation logic.
