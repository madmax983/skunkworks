# Hyperbolic Git

**Hybrid Lineage**: `git-etymology` × `hyperbolic-space`

## Concept

A visualization of Git commit history mapped onto the Poincaré Disk (hyperbolic plane). This allows browsing potentially infinite commit graphs within a finite unit circle.

## Features

- **Hyperbolic Layout**: Commits are laid out in a tree-like structure on the Poincaré Disk.
- **Möbius Navigation**: Changing the focus commit applies a Möbius transformation to the entire space, effectively "moving" the new focus to the center (0,0).
- **Infinite Space**: The layout logic creates a sense of infinite depth as commits approaching the boundary become exponentially smaller.

## Usage

```bash
cargo run -p hyperbolic-git -- <path-to-repo>
```

If no path is provided, it attempts to load the current directory.

## Controls

- **Arrow Keys**: Navigate to neighbor commits (Parent/Child).
- **Enter**: Center the view on the selected commit (Travel).
- **q / Esc**: Quit.

## Lineage Details

- **From `git-etymology`**: Uses `git2` to traverse the commit history DAG.
- **From `hyperbolic-space`**: Uses the `poincare-disk` crate for Möbius transformations and hyperbolic geometry calculations.
- **Novel Trait**: "Time Travel by Hyperbolic Isometry". The act of browsing history is mapped to movement in non-Euclidean space.
