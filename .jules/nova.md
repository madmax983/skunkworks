## [Flux Synthesis]
**Concept:** Use git commit "churn" (insertions + deletions) to modulate the synthesis parameters (FM Synthesis), creating "noise" or "complexity" in the sound for large commits.
**Fate:** In Progress
**Lesson:**

## [Text-Hydro]
**Concept:** Real-time Eulerian fluid simulation in the terminal where density is color and velocity is character direction.
**Fate:** Merged
**Lesson:** Ratatui 0.29.0 deprecated get_mut; use cell_mut. Stable Fluids works surprisingly well in TUI.

## [Mood Lighting]
**Concept:** Synesthetic color generation for git commits. Hash determines Hue, Churn determines Lightness. Visualized in TUI elements.
**Fate:** Merged
**Lesson:** Visual feedback complements audio well in TUI environments.

## [Literary Boids]
**Concept:** A bio-digital ecosystem where Boids consume text characters as food, evolving based on energy intake. Visualized in TUI.
**Fate:** Merged

**Lesson:** Ratatui's `Canvas` widget can render arbitrary text at coordinates, enabling particle systems made of characters.
## [Circadian Rhythm]
**Concept:** Visualize commits based on the time of day they were made (Night, Dawn, Day, Dusk) with corresponding color themes.
**Fate:** Merged
**Lesson:** Simple time-based mapping creates a strong emotional connection to the code history.

## [Git Galaxy]
**Concept:** Force-directed graph visualization of git history in the terminal. Commits are bodies with mass (churn) connected by gravity/springs (ancestry).
**Fate:** Merged
**Lesson:** Physics simulations provide an intuitive "shape" to code history.

## [Syntax Invaders]
**Concept:** A "typing of the dead" style game that scans the local codebase for keywords, which fall from the sky. Players must type them to "refactor" them away.
**Fate:** Merged
**Lesson:** Gamification of codebase familiarity works well. `ratatui` Canvas requires owned Strings for rendering text to avoid lifetime issues.

## [Code Metropolis]
**Concept:** A 3D isometric city visualization of the file system where files are buildings (height = size) and directories are districts. Rendered in TUI using Ratatui Canvas.
**Fate:** Merged
**Lesson:** Isometric projection in terminal requires careful aspect ratio handling (2:1). Recursive Treemap layouts provide a great way to visualize hierarchical data in limited space.

## [Neuro-Terminal]
**Concept:** A TUI-based neural network visualization that trains a simple MLP on a 2D classification problem in real-time.
**Fate:** Merged
**Lesson:** Visualizing the decision boundary on a grid requires careful optimization or subsampling to maintain frame rates in a TUI. Pure Rust ML from scratch is viable for simple visualizations.

## [Fugue State: Code Karaoke]
**Concept:** Visualize the source code while it's being "played" by the synthesizer, highlighting the specific AST nodes (functions, structs) corresponding to the current musical event.
**Fate:** Merged
**Lesson:** Visualizing code structure in sync with audio creates a powerful "synesthetic" experience. Handling `syn` spans and mapping them to TUI lines requires careful coordinate conversion (char indices vs byte indices).
