
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

## [Log Landscape]
**Concept:** A frequency domain visualizer for text streams. It reads from stdin and renders a scrolling 3D wireframe terrain where height matches line intensity/keywords and X-axis is a hash of the content.
**Fate:** Merged
**Lesson:** Transforming textual entropy into physical geometry (terrain) allows for rapid visual pattern recognition in otherwise opaque log streams. `ratatui`'s Canvas lines can effectively create 2.5D wireframe effects.

## [Digital Koi]
**Concept:** A system-monitored Zen Garden where fish (Boids) swim in a pond. Water turbulence is driven by real-time CPU usage, and water clarity by Memory usage.
**Fate:** Merged
**Lesson:** Using system metrics (`sysinfo`) as environmental variables for artificial life simulations creates a calming, organic visualization of machine labor. The Boids algorithm adapts well to `ratatui` Canvas when drawing trails.

## [Process Orbit]
**Concept:** A solar system visualization where the kernel is the Sun and processes are planets. Orbit radius is driven by CPU usage (Gravity), and size by Memory usage.
**Fate:** Merged
**Lesson:** Visualizing invisible system hierarchies as cosmic structures makes resource hogging intuitively obvious (and beautiful). Smooth animation requires stateful integration (`angle += speed * dt`) rather than stateless mapping.

## [Packet Pachinko]
**Concept:** A physics-based "Firewall Construction Kit" where you sort network packets using pinball mechanics. Red "Malware" packets must be blocked, Green "HTTP" packets routed to Port 80, and Blue "SSH" packets to Port 22.
**Fate:** Merged
**Lesson:** TUI Canvas is suitable for simple physics games. Simulating network traffic as physical objects (packets) makes abstract routing concepts tangible and fun. The "Gamification of Sysadmin" has potential.
