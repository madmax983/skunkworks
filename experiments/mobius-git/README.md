# Mobius Git ⚛️🗺️

**"Non-orientable spaces + Version control branching"**

> "We are traversing the history of the repository on a single-sided surface. Merges are not just connections; they are twists in the fabric of the timeline."

## Overview

`mobius-git` visualizes the git commit history as a Directed Acyclic Graph (DAG) mapped onto a **Möbius Strip**.

-   **Time (Depth)** flows along the longitudinal axis ($u$) of the strip.
-   **Branches** diverge along the latitudinal axis ($v$).
-   **Twist**: The strip rotates 180 degrees over one full loop ($2\pi$). Navigating far enough into history (or the future) flips your orientation relative to the "up" vector.

## Controls

-   **Left / Right Arrows**: Move backward/forward in time (scroll along $u$).
-   **Up / Down Arrows**: Move laterally across branches (scroll along $v$).
-   **+ / -**: Zoom in/out (adjust distance from surface).

## Implementation Details

-   **Git Graph**: Uses `git2` to traverse the repository history and build a graph using `petgraph`.
-   **Topological Mapping**: Commits are assigned topological coordinates $(u, v)$ where $u$ is depth and $v$ is a heuristic "lane" for branch tracking.
-   **Möbius Geometry**:
    $$
    x = (R + v \cos(u/2)) \cos(u) \\
    y = (R + v \cos(u/2)) \sin(u) \\
    z = v \sin(u/2)
    $$
-   **Non-Orientable Camera**: The camera's "Up" vector is calculated dynamically from the surface normal, which flips sign after one full revolution, simulating the experience of traversing a non-orientable manifold.

## Dependencies

-   `macroquad`: Rendering engine.
-   `git2`: Libgit2 bindings.
-   `petgraph`: Graph data structure.

## Usage

Run from the root of a git repository:

```bash
cargo run --bin mobius-git
```
