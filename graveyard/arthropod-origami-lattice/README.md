# Arthropod-Origami-Lattice

An experiment crossing `crates/arthropod` (Interactive UI), `crates/origami` (Soft Body Physics), and `crates/miller-lattice` (Hierarchical Trees).

## Lineage 🧬
- **Parent A (`arthropod`)**: Provides the interactive immediate mode graphical user interface and sliders.
- **Parent B (`origami`)**: Provides the procedural soft-body physics engine simulating continuous paper.
- **Parent C (`miller-lattice`)**: Provides the discrete hierarchical tree spatial grouping logic.
- **Novel Trait**: We map the rigid, abstract node distances of `miller-lattice` into the physical structural tension links of `origami`, controlled dynamically via `arthropod` UI interactions. This visualizes a hierarchical layout as a draping, buckling manifold.

## Usage

```bash
# Run with window
cargo run -p arthropod-origami-lattice

# Run headless (for CI or automated testing)
HEADLESS=true cargo run -p arthropod-origami-lattice
```
