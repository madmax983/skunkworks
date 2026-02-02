
## [Biomorph Flow]
**Concept:** A Gray-Scott reaction-diffusion simulation visualized in TUI using `ratatui` Canvas. It supports interactive "seeding" with the mouse and tweaking of Feed/Kill rates to explore pattern space (Coral, Mitosis, Loops).
**Fate:** Merged
**Lesson:** TUI `Canvas` widgets are surprisingly capable of rendering dense 2D fields if you optimize by batching points by color instead of drawing individual pixels. Reaction-diffusion is computationally cheap enough for the main thread.

## [Semantic Spy]
**Concept:** A TUI inspector for `tui-semantic` snapshots. It reads a JSON snapshot from stdin and provides a 3-pane layout: Entity List, Canvas Visualizer, and JSON Details.
**Fate:** Merged
**Lesson:** Simple tools that bridge the gap between "Raw JSON" and "Visual Understanding" are incredibly valuable for debugging. `ratatui` makes building these inspection tools trivial.
