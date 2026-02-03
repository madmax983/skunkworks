# Code Bio Dome 🧬

**Parents**: `experiments/code-crawler` + `experiments/biomorph-flow`

A simulation where functions harvested from your source code become organisms in a biological simulation.

## Concept

This experiment combines static analysis (harvesting function signatures) with a biological simulation. Each function in your codebase becomes an entity in the "Bio Dome".

- **Harvesting**: Scans `.rs` files for function signatures.
- **Simulation**: Functions interact in a 2D world.

## Usage

```bash
cargo run -p code-bio-dome -- [path]
```

- `[path]`: Optional path to scan (defaults to current directory).
- `--semantic`: output semantic snapshot JSON (non-interactive).

## Lineage

- **From `code-crawler`**: Logic for traversing the file system and parsing Rust code (using `syn` or regex).
- **From `biomorph-flow`**: The simulation environment and entity behavior logic.
