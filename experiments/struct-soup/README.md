# Struct Soup 🍜

**Visualize your codebase as a force-directed graph of types.**

Struct Soup scans your Rust code, extracts struct and enum definitions, and finds relationships between them (e.g., `struct Foo { b: Bar }`). It then simulates these relationships as a physical system where:
-   **Structs** are particles.
-   **Fields** are springs.
-   **Repulsion** forces keep them distinct.

## Usage

```bash
cargo run -p struct-soup -- [path_to_scan]
```

If no path is provided, it scans the current directory.

## Controls

-   **WASD / Arrow Keys**: Pan the camera.
-   **+/-**: Zoom in/out.
-   **Space**: Pause/Resume simulation.
-   **R**: Reset view.
-   **Q**: Quit.

## The Theory

Codebases are often complex graphs. By visualizing them as a physical system, we can intuitively spot "God Objects" (heavy nodes with many connections), "Islands" (disconnected types), and tight coupling clusters.
