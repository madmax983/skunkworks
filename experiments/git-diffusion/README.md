# Git-Diffusion 🧬

**Git-Diffusion** is a hybrid experiment created by crossing `git-harmonograph` with `gray-scott`. It visualizes the commit history of a Git repository as an organic, evolving Turing pattern on a Reaction-Diffusion grid.

## Concept

Instead of driving a physical pendulum or moving boids, this experiment uses the metadata of Git commits as localized drops of "chemical catalysts" on a simulated reaction-diffusion surface. The hash of each commit determines the coordinates of the drop, injecting the "V" chemical that triggers the Gray-Scott morphogenesis.

### Lineage
*   **From `git-harmonograph`:** The ability to parse chronological Git commit history and map cryptographic hashes to spatial properties.
*   **From `crates/gray-scott`:** The high-performance, parallelized 2D Gray-Scott reaction-diffusion simulation engine.

## Emergent Behavior
*   **Codebase Morphogenesis:** As you scroll backwards in time (or automatically play), the distinct "spots" of each commit act as seeds. These seeds grow and diffuse, eventually colliding and interacting with the seeds of past commits. The entire history unfolds into a unified, continuous biological fingerprint.
*   The tension between the discrete, unpredictable nature of commit hashes and the continuous, organic growth of Turing patterns reveals how isolated developer actions merge into a coherent repository structure over time.

## Usage

```bash
cargo run -p git-diffusion
```

### Controls
*   **Space**: Drop chemical manually at the center of the grid.
*   **q** / **ESC**: Quit the visualization.
