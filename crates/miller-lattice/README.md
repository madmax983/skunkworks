# Miller Lattice

A procedural generator for visualizing hierarchical data structures (like a file system)
as a 3D crystalline lattice.

This crate maps directories and files into 3D discrete space using Miller indices concepts.
Directories represent structural branch points that alter the normal vector of the growth plane,
while files and subdirectories are placed as `Atom`s around their parent node in a spiral pattern.

## Core Concepts

- **[`LatticePoint`]**: A discrete integer coordinate `(x, y, z)` in 3D space.
- **[`Atom`]**: A single node in the crystal representing a file or directory.
- **[`Crystal`]**: The entire generated structure containing all atoms and their connectivity (bonds).

## Quick Start

```rust
use miller_lattice::Crystal;
use std::path::Path;

fn main() {
    // Build a crystal from a directory path
    let crystal = Crystal::build_from_path(Path::new(".")).unwrap();

    println!("Generated crystal with {} atoms and {} bonds.", crystal.atoms.len(), crystal.bonds.len());
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
miller-lattice = { path = "../miller-lattice" }
```
