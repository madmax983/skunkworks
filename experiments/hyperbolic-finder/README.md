# Hyperbolic Finder ⚛️🗺️

> "Euclidean space is a prison. Break the walls." - Genesis

A file system explorer that maps your directory structure onto a **Poincaré Disk**.

## concept

Traditional file explorers are lists or grids—flat, boring, Euclidean. **Hyperbolic Finder** uses hyperbolic geometry to display an infinite amount of information in a finite space.

- **Focus:** The current directory is at the center (origin).
- **Context:** Children and parents radiate outwards. As they get further away, they shrink exponentially in visual size but remain infinitely accessible.
- **Navigation:** Moving through the file system isn't just scrolling—it's performing **Möbius transformations** on the entire universe.

## Controls

- **Left Click (Node):** Navigate to directory / Select file.
- **Left Click + Drag:** Pan the view (Hyperbolic translation).
- **Right Click:** Go Up (`..`).
- **Backspace:** Reset view to center.

## Features

- **Non-Euclidean Layout:** Nodes are placed in a radial hyperbolic tree.
- **Geodesic Rendering:** Links between nodes are drawn as true hyperbolic geodesics (circular arcs).
- **Git Integration:** Files are colored based on their git status (Green=New, Blue=Modified, etc.).
- **Mass Visualization:** Directory size is visualized as a "halo" around the node.
- **Starfield:** Background stars move correctly according to hyperbolic isometries.

## Technical Details

Built with:
- **Rust** 🦀
- **Macroquad** (Rendering)
- **Poincaré Disk Model** (Math)
- **Git2** (Repository status)

## Running

```bash
cargo run --release
```
