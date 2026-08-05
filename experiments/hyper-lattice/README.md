# hyper-lattice

A hybrid of `hyper-system` and `miller-lattice`.

## Lineage
- **miller-lattice**: Provides the 3D discrete hierarchical crystal structure of the directory tree.
- **hyper-system**: Provides 4D rotation mathematics and system CPU metric interpolation.

## Concept
The discrete hierarchical crystal from `miller-lattice` is mapped into a 4-dimensional space via `hyper-system`. The entire rigid structure undergoes non-Euclidean 4D rotations (XW plane) driven by real-time CPU stress. The resulting 4D points are then projected back down into 3D space for rendering, creating an emergent, folding hyper-crystalline visualization of the codebase that speeds up as the machine is stressed.

## Execution
```bash
# Normal visualization
cargo run -p hyper-lattice --features macroquad_run

# Headless CI mode
cargo run -p hyper-lattice --no-default-features -- --headless
```
