# Git Fluid 🌊🐙

**A Hybrid Experiment by The Splice Surgeon**

> "Commits aren't just points in time; they are magnetic forces shaping the flow of developer gravity."

## 🔬 Experiment Analysis

This experiment crosses the Git history parsing of `git-harmonograph` with the magnetic fluid dynamics simulation of `ferrous-fluid`.

### Concept
Traditional code visualizations treat commit history as static points on a timeline. **Git Fluid** treats developer activity as a physical system of magnetohydrodynamics (MHD).

Commits are mapped onto a 2D space as fixed magnetic poles:
- **Polarity** is determined by the commit author.
- **Strength** is determined by the commit hash.
- **Position** flows sequentially over time.

Fluid particles represent developer flow and focus, constantly being attracted and repulsed by the magnetic footprint of the codebase history.

### Lineage & Genetics
- **Allele A (Data Source)**: `experiments/git-harmonograph` (Git log parsing and entropy mapping).
- **Allele B (Physics)**: `experiments/ferrous-fluid` (Magnetic Particle physics and Platter density grid).
- **Emergent Trait (Codebase Magnetohydrodynamics)**: The particles cluster and flow around high-entropy commits or specific authors, revealing "attractor" developers and the magnetic turbulence of collaborative coding.

## 🕹️ Usage

```bash
cargo run -p git-fluid
```

**Controls**:
- **Q**: Quit the simulation.

## 📊 Technical Details

- **Simulation**: Uses `ferrous-core`'s `Platter` to accumulate fluid density and calculate pressure gradients.
- **Magnetism**: Commits are injected as stationary O(N) forces against the fluid particles.
- **Rendering**: Uses `ratatui`'s Canvas widget for terminal output.