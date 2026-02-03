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
**Lesson:** Transforming textual entropy into physical geometry (terrain) allows for rapid visual pattern recognition in otherwise opaque log streams. `ratatui` Canvas lines can effectively create 2.5D wireframe effects.

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

## [Heap Hopper]
**Concept:** A platformer game where the level is a memory heap. Blocks allocate and free dynamically. The player must jump on valid memory to survive.
**Fate:** Merged
**Lesson:** Gamifying abstract system concepts (memory fragmentation) creates an intuitive understanding of why "fragmentation is bad" (it kills you). `ratatui` is fast enough for simple platformers.

## [Code Sprint]
**Concept:** A "TypeRacer" style game for the codebase. It scans `.rs` files, presents a random snippet, and challenges the user to type it accurately and quickly. Features WPM/Accuracy tracking and real-time diff highlighting.
**Fate:** Merged
**Lesson:** Gamifying "reading the code" forces you to pay attention to syntax and style. `ratatui` is great for overlay-based text interfaces.

## [Genetic Canvas]
**Concept:** An interactive genetic algorithm where the user acts as the fitness function to evolve abstract TUI art (Rects, Lines, Circles).
**Fate:** Merged
**Lesson:** Interactive Evolutionary Computation is a powerful mechanic for exploration-based tools. Users enjoy being the "selector" rather than just a passive observer. `ratatui` Canvas handles dynamic shapes well.

## [System Bonsai]
**Concept:** A TUI visualization of the process tree as a fractal bonsai. Roots are parent processes, branches are children, and leaves are threads/states. CPU usage drives color, Memory drives thickness.
**Fate:** Merged
**Lesson:** Visualizing hierarchical system data as organic structures makes "health" intuitive. `ratatui` Canvas is capable of recursive fractal rendering with decent performance.

## [Git Harmonograph]
**Concept:** A generative art experiment that visualizes git commits as parametric Harmonograph drawings. Frequencies, phases, and damping are derived from commit hashes.
**Fate:** Merged
**Lesson:** Mapping SHA-1 entropy to continuous physical parameters creates unique, deterministic "signatures" for metadata. High-resolution curves (2000+ points) look surprisingly good with `Braille` markers.

## [Fissure Tracker]
**Concept:** A geological simulation of the codebase where dangerous keywords (`unwrap`, `panic`) create physical "stress" in a force-directed graph. High stress causes "fissures" (jagged lines) to crack the bedrock.
**Fate:** Merged
**Lesson:** Visual metaphors for code quality (Stress = Force) are intuitive and instantly readable. `walkdir` + simple string matching is sufficient for powerful MVP visualizations.

## [Git Invaders]
**Concept:** A Space Invaders clone where you battle `git diff` lines. Each added line is an enemy, deletions are red ghosts.
**Fate:** Merged
**Lesson:** Gamifying code review makes the pain of large diffs manageable (or at least destructible). TUI Canvas works well for simple arcade physics.
