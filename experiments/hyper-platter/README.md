# Hyper-Platter

## Concept
A 4D structural topology visualization combined with a fading thermal heat field.

The experiment projects an interactive 4-dimensional hypercube down into a 2D scalar field (`platter`). As the structural constraints of the 4D geometry rotate across higher-dimensional planes (like XW or YW) based on real-time system metrics (CPU load), its vertices scrape against the 2D thermodynamic slice, injecting scalar heat into the field. The result is a fading, glowing topological heatmap visualizing the hidden non-Euclidean structural volume passing through our slice of reality.

## Usage

```bash
cargo run -p hyper-platter
```

Headless bypass for CI:
```bash
cargo run -p hyper-platter -- --headless
```
