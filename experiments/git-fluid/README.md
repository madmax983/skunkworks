# Git Fluid 🧲🌊

**A Hybrid Experiment by The Splice Surgeon**

> "The magnetic pulse of your codebase history."

## 🔬 Experiment Analysis

This experiment combines Git commit history parsing with magnetic fluid dynamics.

- **Parent A**: `experiments/git-harmonograph` (Git History Parsing & TUI)
- **Parent B**: `experiments/ferrous-fluid` (Ferrous Fluid Dynamics & TUI)

### Concept
`git-fluid` asks: *What if the history of a codebase exerted a physical magnetic force?*

As you navigate through the git history of a project, the SHA-1 hash of each commit is parsed to calculate the coordinates and polarity (North/South) of a new magnet. These magnets are dropped into a 2D ferrous fluid simulation where particles react to gravity, boundaries, fluid pressure, and the magnetic field.

### Lineage & Genetics
- **Allele A (Data Source)**: Git commit metadata extraction from `git-harmonograph`.
- **Allele B (Physics Engine)**: Magnetic and density-pressure forces from `ferrous-fluid` using `ferrous-core`.
- **Emergent Trait**: Magnetic Codebase Fingerprint. The git history creates an evolving, complex magnetic pressure field. The particles cluster, spike, and dance in a visual representation of the commit's hash entropy.

## 🕹️ Controls

- **Left / h**: Previous commit in history
- **Right / l**: Next commit in history
- **R**: Reset the fluid simulation
- **Q / Esc**: Quit

## 📊 Technical Details

- **Physics**: Hybrid Lagrangian-Eulerian approach computing magnetic attraction/repulsion and grid-based fluid pressure.
- **Data Integration**: The first 6 characters of the SHA-1 hash determine the X/Y coordinates and the polarity. To avoid overwhelming the fluid, only the latest 3 commit magnets are kept active at a time.
- **Rendering**: `ratatui` Canvas.