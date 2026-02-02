
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

## [Git Rogue]
**Concept:** A text-based roguelike where the map is the Git commit history. Commits are rooms, parents/children are exits, and bugs/features are encounters determined by commit messages.
**Fate:** Merged
**Lesson:** Interpreting version control graphs as physical spaces (dungeons) creates a natural exploration mechanic. Bidirectional graph traversal requires pre-processing or double-linking logic since Git is natively directed acyclic (backwards).

## [Cargo Compass]
**Concept:** A TUI explorer and launcher for the Cargo workspace. It visualizes the dependency graph (list view) and allows running experiments directly from the interface.
**Fate:** Merged
**Lesson:** `cargo_metadata` + `ratatui` + `tui-shared` creates a powerful "Dashboard" pattern. Suspending the TUI to run a subprocess (`cargo run`) works seamlessly if raw mode is handled correctly.
