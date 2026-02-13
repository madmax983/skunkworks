# Ferrous Graph

**Genetic Cross**: `newtonian-graph` x `ferrous-sector`

A force-directed graph visualization of the codebase where nodes (files) are magnetic particles floating over a decaying magnetic platter (grid).

## Phenotype
- **Magnetic Hysteresis**: Nodes leave magnetic trails on the background grid.
- **Path Dependence**: Nodes are attracted to high-magnetism areas, causing them to follow established paths (or get stuck in "rust").
- **Bit Rot**: The background magnetism decays over time.

## Usage
Run with `cargo run -p ferrous-graph`.
Controls:
- `+/-`: Zoom
- `Arrows`: Pan
- `R`: Reset view
- `Q`: Quit

## Lineage
- **Structure**: `newtonian-graph` (Physics engine, graph scanning)
- **Substrate**: `ferrous-sector` (Magnetic grid, decay logic)
