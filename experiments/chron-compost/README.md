# 🧬 Chron Compost

**"Time Scavengers consuming the ancient bedrock."**

A hybrid experiment combining **Chrontext** (Git blame timeline visualization) and **Compost Chimera** (Chimera VM biological agents feeding on text).

## 🧬 Lineage

- **Parent A**: `experiments/chrontext` (The Environment)
  - Provided the `BlameAnalyzer` which uses `git2` to analyze the age of lines of code.
  - Provided the visual rendering mapping age scores to colors (Cold/Warm/Hot).

- **Parent B**: `experiments/compost-chimera` (The Life)
  - Provided the `ChimeraVM` agents.
  - Provided the core TUI simulation loop.

## 🧪 Phenotype

**Time Scavengers**. Small agents (`@`) roam across the buffers of your source code. The code itself is mapped to its `git blame` history, where older code is colored blue/gray ("Cold") and newly committed code is yellow ("Hot").

The agents feed on the "Cold" lines of code—the ancient bedrock of the codebase. As they consume it, they "refactor" the codebase history, resetting the age score of the line back to 1.0 (Hot/Fresh) and gaining energy in the process.

## 🎮 Controls

- `j/k/Down/Up`: Scroll the viewport.
- `r`: Respawn agents.
- `q/Esc`: Quit.

## 🏗️ Architecture

The simulation runs a TUI loop where `Simulation` manages a population of `Agent`s. Each agent runs an internal `ChimeraVM`. During each tick, the environment feeds the `age_score` of the current line to the agent's stack. If the `age_score` is low enough (old code), the agent "eats" it, mutating the underlying `blame_info` struct to reset the age score and simulate a refactoring event.
