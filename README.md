# Skunkworks 🧪⚛️

**A Living Archive of AI-Generated Experimental Code**

48 Jules bots. 48 unique personas. 2 run every hour. Each with a different obsession, a different lens through which to see creative coding.

## What Is This?

This repository is an autonomous creative coding laboratory. Every hour, 2 AI agents from a pool of 48—each with a distinct persona and creative directive—generate experimental code projects. Some simulate entropy, others chase emergent behavior, visualize data, or explore the boundaries of computation itself.

This isn't a curated portfolio. It's a raw, unfiltered stream of algorithmic creativity.

## The Bots

Each bot operates from a unique template. Here's one example:

**Template 23: The Archivist 📚**
> Obsessed with information decay, digital rot, and data archaeology. Simulates entropy—bit rot, format degradation, the slow death of data—and perhaps, resurrection.

The other 47 bots have equally specific obsessions:
- Emergence and complexity
- Neural networks and learning
- Physics simulations
- Procedural generation
- Data visualization
- Glitch art and corruption
- Graph theory and networks
- Temporal dynamics
- Quantum metaphors
- Organic growth patterns
- *...and 37 more*

## Core Infrastructure

While the experiments are diverse, they share a common foundation located in `crates/`:

*   **`tui-shared`**: A robust wrapper for `ratatui` terminal initialization. It handles the "boilerplate" of entering raw mode, setting up alternate screens, and ensuring graceful cleanup (even on panic).
*   **`tui-semantic`**: A framework-agnostic bridge for AI interaction. It allows TUI applications to export their "semantic state" (entities, metrics, game state) as structured JSON, enabling LLMs to "see" and "play" the applications without parsing pixels.

## How It Works

**Schedule:** 2 bots run every hour, cycling through all 48 personas
**Process:** Each bot receives its template prompt + a "What If" scenario
**Output:** Rust code experiments, often terminal-based but not always
**Methodology:** Most follow a variation of RED-GREEN-REFACTOR or similar creative workflows

## Experiments Generated

The `experiments/` directory contains the artifacts:

```
experiments/
├── automata-warfare/        # Cellular automata battles
├── code-metropolis/         # Codebase as 3D city
├── git_galaxy/              # Git history as force graph
├── git_rhythm/              # Commits as music
├── hyphal-commute/          # Mycelial growth simulation
├── literary-boids/          # Flocking text
├── neuro-terminal/          # Neural network visualization
├── schrodingers-text/       # Quantum text behavior
├── syntax-invaders/         # Code-themed arcade
├── term-fluids/             # Fluid dynamics
├── text-hydro/              # Water flow ASCII
└── ...                      # More experiments emerge hourly
```

Each experiment is self-contained and may reflect the obsessions of multiple bot personas over time.

## Philosophy

**Autonomous Creativity:** The bots don't wait for prompts. They generate, explore, fail, and iterate.

**Diverse Lenses:** 48 different ways of seeing the same creative space produces emergent variety impossible from a single perspective.

**Digital Artifacts:** This is archaeology in reverse—watching the sediment layers of AI creativity accumulate in real-time.

**No Curation:** Raw output. Some experiments thrive, some fail. Both are valuable.

## Running Experiments

This is a Cargo workspace:

```bash
# Build everything
cargo build --workspace

# Run a specific experiment
cargo run -p neuro-terminal
cargo run -p git_galaxy

# List all workspace members
cargo metadata --no-deps | grep name
```

## The Archive Grows

Every hour. Every day. 48 perspectives. Endless permutations.

Some experiments will be refined over multiple iterations as bots revisit them. Others will be one-shot explorations. All contribute to the living archive.

## Meta

**Language:** Rust (primarily)
**Common Stack:** ratatui, crossterm, rand, anyhow
**Platforms:** Terminal-first, but not exclusive
**Quality:** Experimental (⚛️) by definition

---

*"What if we simulated 1000 years of bit rot on a Git repository—watching history fade, commits become unreadable, trying to reconstruct what was lost?"*
— Template 23: The Archivist

*48 bots. 2 per hour. Infinite permutations.*
